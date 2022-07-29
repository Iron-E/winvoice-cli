use core::fmt::Display;

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
use clinvoice_config::{Config, Error};
use clinvoice_match::{
	MatchEmployee,
	MatchInvoice,
	MatchJob,
	MatchOption,
	MatchOrganization,
	MatchTimesheet,
};
use clinvoice_schema::{ContactKind, RestorableSerde};
use futures::{stream, TryStreamExt};
use serde::{de::DeserializeOwned, Serialize};
use sqlx::{Database, Executor, Pool, Transaction};

use super::{Update, UpdateCommand};
use crate::{args::RunAction, fmt, input, utils::Identifiable, DynResult};

#[async_trait::async_trait(?Send)]
impl RunAction for Update
{
	async fn action<CAdapter, EAdapter, JAdapter, LAdapter, OAdapter, TAdapter, XAdapter, TDb>(
		self,
		connection: Pool<TDb>,
		config: Config,
	) -> DynResult<()>
	where
		CAdapter: Deletable<Db = TDb> + ContactAdapter,
		EAdapter: Deletable<Db = TDb> + EmployeeAdapter,
		JAdapter: Deletable<Db = TDb> + JobAdapter,
		LAdapter: Deletable<Db = TDb> + LocationAdapter,
		OAdapter: Deletable<Db = TDb> + OrganizationAdapter,
		TAdapter: Deletable<Db = TDb> + TimesheetAdapter,
		XAdapter: Deletable<Db = TDb> + ExpensesAdapter,
		TDb: Database,
		for<'connection> &'connection mut TDb::Connection: Executor<'connection, Database = TDb>,
		for<'connection> &'connection mut Transaction<'connection, TDb>:
			Executor<'connection, Database = TDb>,
	{
		/// A generic deletion function which works for any of the provided adapters in the outer
		/// function, as they all implement `TUpdatable` at the minimum.
		async fn update<TUpdatable, TDb>(
			connection: &Pool<TDb>,
			entities: &mut [TUpdatable::Entity],
		) -> DynResult<()>
		where
			TDb: Database,
			TUpdatable: Updatable<Db = TDb>,
			TUpdatable::Entity:
				Clone + DeserializeOwned + Display + Identifiable + RestorableSerde + Serialize + Sync,
			for<'connection> &'connection mut Transaction<'connection, TDb>:
				Executor<'connection, Database = TDb>,
		{
			#[rustfmt::skip]
			entities.iter_mut().try_for_each(|e| {
				*e = input::edit_and_restore(e, format!(
					"Make any desired edits to the {}",
					fmt::type_name::<TUpdatable::Entity>()
				))?;

				input::Result::Ok(())
			})?;

			let mut transaction = connection.begin().await?;

			TUpdatable::update(
				&mut transaction,
				entities.iter().inspect(|e| Update::report_updated(*e)),
			)
			.await?;

			transaction.commit().await?;
			Ok(())
		}

		match self.command
		{
			UpdateCommand::Contact =>
			{
				let mut selected = input::select_retrieved::<CAdapter, _, _>(
					&connection,
					None,
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
			UpdateCommand::Expense =>
			{
				let mut selected = input::select_retrieved::<XAdapter, _, _>(
					&connection,
					None,
					"Query the Expenses to update",
				)
				.await?;

				#[rustfmt::skip]
				stream::iter(selected.iter_mut().filter_map(|expense| input::confirm_then_some(
					format!("Do you want to change the Timesheet of {expense}?"),
					Ok(expense),
				)))
				.try_for_each(|expense| {
					let connection = &connection;

					async {
						expense.timesheet_id = input::select_one_retrieved::<TAdapter, _, _>(
							connection,
							None,
							"Query the Timesheet to attach this Expense to",
						)
						.await
						.map(|t| t.id)?;

						DynResult::Ok(())
					}
				})
				.await?;

				update::<XAdapter, _>(&connection, &mut selected).await?;
			},
			UpdateCommand::Employee { default } =>
			{
				let match_condition = default
					.then(|| {
						config
							.employees
							.id
							.map(MatchEmployee::from)
							.ok_or_else(|| Error::NotConfigured("id".into(), "employees".into()))
					})
					.transpose()?;

				let mut selected = input::select_retrieved::<EAdapter, _, _>(
					&connection,
					match_condition,
					"Query the Employees to update",
				)
				.await?;

				update::<EAdapter, _>(&connection, &mut selected).await?;
			},
			UpdateCommand::Location =>
			{
				let mut selected = input::select_retrieved::<LAdapter, _, _>(
					&connection,
					None,
					"Query the Locations to update",
				)
				.await?;

				#[rustfmt::skip]
				stream::iter(selected.iter_mut().filter_map(|location| input::confirm_then_some(
					format!("Do you want to put {} into a new Location", fmt::quoted(&location.name)),
					Ok(location),
				)))
				.try_for_each(|location| {
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

						DynResult::Ok(())
					}
				})
				.await?;

				update::<LAdapter, _>(&connection, &mut selected).await?;
			},
			UpdateCommand::Job {
				close,
				invoice_paid,
				reopen,
			} =>
			{
				#[rustfmt::skip]
				let mut selected = input::select_retrieved::<JAdapter, _, _>(
					&connection,
					(close || reopen)
						.then(|| MatchJob {
							date_close: close.then_some(MatchOption::None).unwrap_or_else(MatchOption::some),
							..Default::default()
						})
						.or_else(|| invoice_paid.then(|| MatchJob {
							invoice: MatchInvoice {
								date_issued: MatchOption::some(),
								..Default::default()
							},
							..Default::default()
						})),
					"Query the Jobs to update",
				)
				.await?;

				#[rustfmt::skip]
				stream::iter(selected.iter_mut().filter_map(|job| input::confirm_then_some(
					format!(
						"Do you want to change the client {} of Job {} ({})?",
						fmt::quoted(&job.client.name),
						fmt::id_num(job.id),
						job.objectives
							.lines()
							.next()
							.expect("Job should have at least one line of description"),
					),
					Ok(job),
				)))
				.try_for_each(|job| {
					let connection = &connection;

					async {
						job.client = input::select_one_retrieved::<OAdapter, _, _>(
							connection,
							None,
							"Query the Organization you want to set this Job's client to",
						)
						.await?;

						DynResult::Ok(())
					}
				})
				.await?;

				update::<JAdapter, _>(&connection, &mut selected).await?;
			},
			UpdateCommand::Organization { employer } =>
			{
				let match_condition = employer
					.then(|| {
						config
							.organizations
							.employer_id
							.map(MatchOrganization::from)
							.ok_or_else(|| {
								Error::NotConfigured("employer_id".into(), "organizations".into())
							})
					})
					.transpose()?;

				let mut selected = input::select_retrieved::<OAdapter, _, _>(
					&connection,
					match_condition,
					"Query the Organizations to update",
				)
				.await?;

				#[rustfmt::skip]
				stream::iter(selected.iter_mut().filter_map(|organization| input::confirm_then_some(
					format!("Do you want to change the Location of {}?", fmt::quoted(&organization.name)),
					Ok(organization),
				)))
				.try_for_each(|organization| {
					let connection = &connection;

					async {
						organization.location = input::select_one_retrieved::<LAdapter, _, _>(
							connection,
							None,
							"Query the Location you want to move this Organization to",
						)
						.await?;

						DynResult::Ok(())
					}
				})
				.await?;

				update::<OAdapter, _>(&connection, &mut selected).await?;
			},
			UpdateCommand::Timesheet { restart, stop } =>
			{
				#[rustfmt::skip]
				let mut selected = input::select_retrieved::<TAdapter, _, _>(
					&connection,
					(restart || stop).then(|| MatchTimesheet {
						time_end: stop.then_some(MatchOption::None).unwrap_or_else(MatchOption::some),
						..Default::default()
					}),
					"Query the Timesheets to update",
				)
				.await?;

				todo!("Prompt to change employee");
				todo!("Prompt to change job");

				update::<TAdapter, _>(&connection, &mut selected).await?;
			},
		};

		Ok(())
	}
}
