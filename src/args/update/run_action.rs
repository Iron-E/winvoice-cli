use core::fmt::Display;
use std::collections::HashMap;

use clinvoice_adapter::{
	schema::{
		ContactAdapter,
		EmployeeAdapter,
		ExpensesAdapter,
		JobAdapter,
		LocationAdapter,
		OrganizationAdapter,
		TimesheetAdapter,
	},
	Deletable,
	Updatable,
};
use clinvoice_config::Config;
use clinvoice_schema::{ContactKind, InvoiceDate, RestorableSerde};
use futures::{stream, Future, TryStreamExt};
use serde::{de::DeserializeOwned, Serialize};
use sqlx::{Database, Executor, Pool, Transaction};

use super::{Update, UpdateCommand};
use crate::{
	args::RunAction,
	fmt,
	input::{self, expense},
	utils::Identifiable,
	DynResult,
};

#[async_trait::async_trait(?Send)]
impl RunAction for Update
{
	async fn action<CAdapter, EAdapter, JAdapter, LAdapter, OAdapter, TAdapter, XAdapter, Db>(
		self,
		connection: Pool<Db>,
		config: Config,
	) -> DynResult<()>
	where
		CAdapter: Deletable<Db = Db> + ContactAdapter,
		EAdapter: Deletable<Db = Db> + EmployeeAdapter,
		JAdapter: Deletable<Db = Db> + JobAdapter,
		LAdapter: Deletable<Db = Db> + LocationAdapter,
		OAdapter: Deletable<Db = Db> + OrganizationAdapter,
		TAdapter: Deletable<Db = Db> + TimesheetAdapter,
		XAdapter: Deletable<Db = Db> + ExpensesAdapter,
		Db: Database,
		for<'connection> &'connection mut Db::Connection: Executor<'connection, Database = Db>,
		for<'connection> &'connection mut Transaction<'connection, Db>:
			Executor<'connection, Database = Db>,
	{
		/// Uses [`Iterator::filter_map`] to filter out items of `iter` which return [`None`] from
		/// [`input::confirm_then_some`], otherwise mapping
		async fn filter_then_try_for_each<'input, Iter, Input, PromptFn, Prompt, TryFn, TryFnFut>(
			iter: Iter,
			prompt: PromptFn,
			try_fn: TryFn,
		) -> DynResult<()>
		where
			Input: 'input,
			Iter: Iterator<Item = &'input mut Input>,
			Prompt: Into<String>,
			PromptFn: Fn(&Input) -> Prompt,
			TryFn: Fn(&'input mut Input) -> TryFnFut,
			TryFnFut: Future<Output = DynResult<()>>,
		{
			stream::iter(
				iter.filter_map(move |item| input::confirm_then_some(prompt(item), Ok(item))),
			)
			.try_for_each(try_fn)
			.await
		}

		/// Gets the first line of any given [`&str`] `s`.
		fn first_line(s: &str) -> &str
		{
			s.lines().next().unwrap()
		}

		/// A generic deletion function which works for any of the provided adapters in the outer
		/// function, as they all implement `Updatable` at the minimum.
		async fn update<Upd, Db>(
			connection: &Pool<Db>,
			entities: &mut [Upd::Entity],
		) -> DynResult<()>
		where
			Db: Database,
			Upd: Updatable<Db = Db>,
			Upd::Entity: Clone
				+ DeserializeOwned
				+ Display
				+ Identifiable
				+ RestorableSerde
				+ Serialize
				+ Sync,
			for<'connection> &'connection mut Transaction<'connection, Db>:
				Executor<'connection, Database = Db>,
		{
			#[rustfmt::skip]
			entities.iter_mut().try_for_each(|e| {
				*e = input::edit_and_restore(e, format!(
					"Make any desired edits to the {}",
					fmt::type_name::<Upd::Entity>()
				))?;

				input::Result::Ok(())
			})?;

			let mut transaction = connection.begin().await?;

			Upd::update(&mut transaction, entities.iter().inspect(|e| Update::report_updated(*e)))
				.await?;

			transaction.commit().await?;
			Ok(())
		}

		match self.command
		{
			UpdateCommand::Contact =>
			{
				let match_condition = self.match_args.try_into()?;
				let mut selected = input::select_retrieved::<CAdapter, _, _>(
					&connection,
					match_condition,
					"Query the Contacts to update",
				)
				.await?;

				#[rustfmt::skip]
				stream::iter(selected.iter_mut().filter_map(|contact| match contact.kind
				{
					ContactKind::Address(_) => input::confirm_then_some(
						format!("Do you want to change the location of {}?", fmt::quoted(&contact.label)),
						Ok(contact),
					),
					_ => None,
				}))
				.try_for_each(|contact| {
					let connection = &connection;
					async {
						contact.kind = input::select_one_retrieved::<LAdapter, _, _>(
							connection,
							None,
							"Query the Location you want to set this address to",
						)
						.await
						.map(ContactKind::Address)?;

						DynResult::Ok(())
					}
				})
				.await?;

				update::<CAdapter, _>(&connection, &mut selected).await?;
			},

			UpdateCommand::Employee { default } =>
			{
				let match_condition = match default
				{
					false => self.match_args.try_into()?,
					true => config.employees.id_or_err().map(|id| Some(id.into()))?,
				};

				let mut selected = input::select_retrieved::<EAdapter, _, _>(
					&connection,
					match_condition,
					"Query the Employees to update",
				)
				.await?;

				update::<EAdapter, _>(&connection, &mut selected).await?;
			},

			UpdateCommand::Expense =>
			{
				let match_condition = self.match_args.try_into()?;
				let mut selected = input::select_retrieved::<XAdapter, _, _>(
					&connection,
					match_condition,
					"Query the Expenses to update",
				)
				.await?;

				#[rustfmt::skip]
				filter_then_try_for_each(
					selected.iter_mut(),
					|x| format!("Do you want to change the Timesheet of {x}?"),
					|x| {
						let connection = &connection;
						async {
							x.timesheet_id = input::select_one_retrieved::<TAdapter, _, _>(
								connection,
								None,
								"Query the Timesheet to attach this Expense to",
							)
							.await
							.map(|t| t.id)?;

							Ok(())
						}
					},
				)
				.await?;

				update::<XAdapter, _>(&connection, &mut selected).await?;
			},

			UpdateCommand::Location =>
			{
				let match_condition = self.match_args.try_into()?;
				let mut selected = input::select_retrieved::<LAdapter, _, _>(
					&connection,
					match_condition,
					"Query the Locations to update",
				)
				.await?;

				#[rustfmt::skip]
				filter_then_try_for_each(
					selected.iter_mut(),
					|l| format!(
						"Do you want to put {} into a new Location",
						fmt::quoted(&l.name),
					),
					|location| {
						let connection = &connection;
						async {
							location.outer = input::select_one_retrieved::<LAdapter, _, _>(
								connection,
								None,
								format!(
									"Query the Location you want to put {} inside of",
									location.name,
								),
							)
							.await
							.map(|l| Some(l.into()))?;

							Ok(())
						}
					},
				)
				.await?;

				update::<LAdapter, _>(&connection, &mut selected).await?;
			},

			UpdateCommand::Job { close, invoice_issued, invoice_paid, reopen } =>
			{
				let match_condition = self.match_args.try_into()?;
				let mut selected = input::select_retrieved::<JAdapter, _, _>(
					&connection,
					match_condition,
					"Query the Jobs to update",
				)
				.await?;

				#[rustfmt::skip]
				filter_then_try_for_each(
					selected.iter_mut(),
					|j| format!(
						"Do you want to change the client {} of Job {} ({})?",
						fmt::quoted(&j.client.name),
						fmt::id_num(j.id),
						first_line(&j.objectives),
					),
					|j| {
						let connection = &connection;
						async {
							j.client = input::select_one_retrieved::<OAdapter, _, _>(
								connection,
								None,
								"Query the Organization you want to set this Job's client to",
							)
							.await?;

							Ok(())
						}
					},
				).await?;

				if !(close.flag() || invoice_issued.flag() || invoice_paid.flag() || reopen)
				{
					return update::<JAdapter, _>(&connection, &mut selected).await;
				}

				let close_arg = close.iff_flagged_utc_or_now();
				let issued_arg = invoice_issued.iff_flagged_utc_or_now();
				let paid_arg = invoice_paid.iff_flagged_utc_or_now();

				for s in &mut selected
				{
					if reopen
					{
						s.date_close = None;
						s.invoice.date = None;
						continue;
					}

					if close_arg.is_some()
					{
						s.date_close = close_arg;
						let expenses = expense::menu()?;
						XAdapter::create(&connection, expenses, s.id).await?;
					}

					if let Some(arg) = issued_arg
					{
						s.invoice.date = Some(InvoiceDate { issued: arg, paid: None });
					}

					if let Some((_, date)) = paid_arg.zip(s.invoice.date)
					{
						s.invoice.date = Some(InvoiceDate { paid: paid_arg, ..date });
					}
				}

				let mut transaction = connection.begin().await?;
				JAdapter::update(
					&mut transaction,
					selected.iter().inspect(|e| Self::report_updated(*e)),
				)
				.await?;

				transaction.commit().await?;
			},

			UpdateCommand::Organization { employer } =>
			{
				let match_condition = match employer
				{
					false => self.match_args.try_into()?,
					true => config.organizations.employer_id_or_err().map(|id| Some(id.into()))?,
				};

				let mut selected = input::select_retrieved::<OAdapter, _, _>(
					&connection,
					match_condition,
					"Query the Organizations to update",
				)
				.await?;

				#[rustfmt::skip]
				filter_then_try_for_each(
					selected.iter_mut(),
					|o| format!("Do you want to change the Location of {}?", fmt::quoted(&o.name)),
					|o| {
						let connection = &connection;
						async {
							o.location = input::select_one_retrieved::<LAdapter, _, _>(
								connection,
								None,
								"Query the Location you want to move this Organization to",
							)
							.await?;

							Ok(())
						}
					},
				)
				.await?;

				update::<OAdapter, _>(&connection, &mut selected).await?;
			},

			UpdateCommand::Timesheet { restart, stop } =>
			{
				let match_condition = self.match_args.try_into()?;
				let mut selected = input::select_retrieved::<TAdapter, _, _>(
					&connection,
					match_condition,
					"Query the Timesheets to update",
				)
				.await?;

				#[rustfmt::skip]
				filter_then_try_for_each(
					selected.iter_mut(),
					|t| format!(
						"Do you want to change the employee {} attached to Timesheet {} ({})?",
						fmt::quoted(&t.employee.name),
						fmt::id_num(t.id),
						first_line(&t.work_notes),
					),
					|t| {
						let connection = &connection;
						async {
							t.employee = input::select_one_retrieved::<EAdapter, _, _>(
								connection,
								None,
								"Query the Employee you want to assign to this Timesheet",
							)
							.await?;

							Ok(())
						}
					},
				)
				.await?;

				#[rustfmt::skip]
				filter_then_try_for_each(
					selected.iter_mut(),
					|t| format!(
						"Do you want to change the job {} ({}) that Timesheet {} ({}) is assigned to?",
						fmt::id_num(t.job.id),
						first_line(&t.job.objectives),
						fmt::id_num(t.id),
						first_line(&t.work_notes),
					),
					|t| {
						let connection = &connection;
						async {
							t.job = input::select_one_retrieved::<JAdapter, _, _>(
								connection,
								None,
								"Query the Job you want to assign this Timesheet to",
							)
							.await?;

							Ok(())
						}
					},
				)
				.await?;

				if !(restart.flag() || stop.flag())
				{
					return update::<TAdapter, _>(&connection, &mut selected).await;
				}

				let restart_arg = restart.iff_flagged_utc_or_now();
				let stop_arg = stop.iff_flagged_utc_or_now();

				selected.iter_mut().for_each(|s| {
					if let Some(arg) = restart_arg
					{
						s.time_begin = arg;
					}
					else if stop_arg.is_some()
					{
						s.time_end = stop_arg;
					}
				});

				let mut transaction = connection.begin().await?;
				TAdapter::update(
					&mut transaction,
					selected.iter().inspect(|e| Self::report_updated(*e)),
				)
				.await?;

				transaction.commit().await?;
			},
		};

		Ok(())
	}
}
