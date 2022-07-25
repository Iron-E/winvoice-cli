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
use clinvoice_schema::RestorableSerde;
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
		for<'c> &'c mut TDb::Connection: Executor<'c, Database = TDb>,
		for<'c> &'c mut Transaction<'c, TDb>: Executor<'c, Database = TDb>,
	{
		/// A generic deletion function which works for any of the provided adapters in the outer
		/// function, as they all implement `TUpdatable` at the minimum.
		async fn update<TUpdatable, TDb, TMatch>(
			connection: Pool<TDb>,
			entities: &mut [TUpdatable::Entity],
		) -> DynResult<()>
		where
			TDb: Database,
			TUpdatable: Updatable<Db = TDb>,
			TUpdatable::Entity:
				Clone + DeserializeOwned + Display + Identifiable + RestorableSerde + Serialize + Sync,
			for<'c> &'c mut TDb::Connection: Executor<'c, Database = TDb>,
			for<'c> &'c mut Transaction<'c, TDb>: Executor<'c, Database = TDb>,
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
				entities.iter().map(|e| {
					Update::report_updated(e);
					e
				}),
			)
			.await?;

			transaction.commit().await?;
			Ok(())
		}

		match self.command
		{
			UpdateCommand::Contact =>
			{
				todo!();
			},
			UpdateCommand::Expense =>
			{
				todo!();
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

				todo!();
			},
			UpdateCommand::Location =>
			{
				todo!();
			},
			UpdateCommand::Job {
				close,
				invoice_issued,
				invoice_paid,
				reopen,
			} =>
			{
				#[rustfmt::skip]
				let match_condition = (close || reopen).then(|| MatchJob {
						date_close: close
							.then_some(MatchOption::None)
							.unwrap_or_else(|| MatchOption::Not(Box::new(None.into()))),
						..Default::default()
					})
					.or_else(|| invoice_issued.then(|| MatchJob {
						invoice: MatchInvoice {
							date_paid: None.into(),
							..Default::default()
						},
						..Default::default()
					}))
					.or_else(|| invoice_paid.then(|| MatchJob {
						invoice: MatchInvoice {
							date_issued: MatchOption::Not(Box::new(None.into())),
							..Default::default()
						},
						..Default::default()
					}));

				todo!();
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

				todo!();
			},
			UpdateCommand::Timesheet { restart, stop } =>
			{
				let match_condition = (restart || stop).then(|| MatchTimesheet {
					time_end: stop
						.then_some(MatchOption::None)
						.unwrap_or_else(|| MatchOption::Not(Box::new(None.into()))),
					..Default::default()
				});

				let selected = input::select_retrieved::<TAdapter, _, _>(
					&connection,
					match_condition,
					"Query the Timesheets to update",
				)
				.await?;

				todo!();
			},
		};

		Ok(())
	}
}
