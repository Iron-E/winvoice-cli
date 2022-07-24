use core::fmt::Display;
use std::error::Error as StdError;

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
	Retrievable,
};
use clinvoice_config::{Config, Error};
use clinvoice_finance::ExchangeRates;
use clinvoice_match::{MatchOrganization, MatchTimesheet};
use futures::{future, stream, TryFutureExt, TryStreamExt};
use serde::{de::DeserializeOwned, Serialize};
use sqlx::{Database, Executor, Pool};
use tokio::fs;

use super::{Retrieve, RetrieveCommand};
use crate::{args::RunAction, fmt, input, DynResult};

#[async_trait::async_trait(?Send)]
impl RunAction for Retrieve
{
	async fn action<CAdapter, EAdapter, JAdapter, LAdapter, OAdapter, TAdapter, XAdapter, TDb>(
		self,
		connection: Pool<TDb>,
		config: Config,
	) -> DynResult<()>
	where
		TDb: Database,
		CAdapter: Deletable<Db = TDb> + ContactAdapter,
		EAdapter: Deletable<Db = TDb> + EmployeeAdapter,
		JAdapter: Deletable<Db = TDb> + JobAdapter,
		LAdapter: Deletable<Db = TDb> + LocationAdapter,
		OAdapter: Deletable<Db = TDb> + OrganizationAdapter,
		TAdapter: Deletable<Db = TDb> + TimesheetAdapter,
		XAdapter: Deletable<Db = TDb> + ExpensesAdapter,
		for<'c> &'c mut TDb::Connection: Executor<'c, Database = TDb>,
	{
		/// [`Display`] every element of some `array` using [`println!`].
		fn print_all<T>(array: &[T])
		where
			T: Display,
		{
			array.iter().for_each(|element| {
				println!("{element}");
			});
		}

		/// A generic deletion function which works for any of the provided adapters in the outer
		/// function, as they all implement `TDelRetrievable` at the minimum.
		async fn retrieve<TRetrievable, TDb, TMatch>(
			connection: &Pool<TDb>,
			match_condition: TMatch,
			print: bool,
		) -> DynResult<Vec<TRetrievable::Entity>>
		where
			TDb: Database,
			TMatch: TryInto<Option<TRetrievable::Match>>,
			TMatch::Error: 'static + StdError,
			TRetrievable: Retrievable<Db = TDb>,
			TRetrievable::Entity: Clone + Display + Sync,
			TRetrievable::Match: Default + DeserializeOwned + Serialize,
			for<'c> &'c mut TDb::Connection: Executor<'c, Database = TDb>,
		{
			let retrieved = match match_condition.try_into()?
			{
				Some(condition) => TRetrievable::retrieve(&connection, &condition).await?,

				#[rustfmt::skip]
				_ => input::retrieve::<TRetrievable, _, _>(
					&connection,
					format!("Query the {} to delete", fmt::type_name::<TRetrievable::Entity>()),
				)
				.await?,
			};

			if print
			{
				print_all(&retrieved);
			}

			Ok(retrieved)
		}

		match self.command
		{
			RetrieveCommand::Contact =>
			{
				retrieve::<CAdapter, _, _>(&connection, self.match_args, true).await?;
			},

			RetrieveCommand::Employee {
				default,
				set_default,
			} =>
			{
				let match_condition = match default
				{
					false => self.match_args.try_into()?,
					_ => config
						.employees
						.id
						.ok_or_else(|| Error::NotConfigured("id".into(), "employees".into()))
						.map(|id| Some(id.into()))?,
				};

				let retrieved =
					retrieve::<EAdapter, _, _>(&connection, match_condition, !set_default).await?;

				if set_default
				{
					let selected =
						input::select_one(&retrieved, "Select the Employee to set as the default")?;
					let mut c = config;
					c.employees.id = Some(selected.id);
					c.write()?;
				}
			},

			RetrieveCommand::Expense =>
			{
				retrieve::<XAdapter, _, _>(&connection, self.match_args, true).await?;
			},

			RetrieveCommand::Job {
				export,
				format,
				output_dir,
			} =>
			{
				let retrieved =
					retrieve::<JAdapter, _, _>(&connection, self.match_args, !export).await?;

				if export
				{
					let match_all_contacts = Default::default();
					let match_employer = config
						.organizations
						.employer_id
						.map(MatchOrganization::from)
						.ok_or_else(|| {
							Error::NotConfigured("employer_id".into(), "organizations".into())
						})?;

					let exchange_rates_fut = ExchangeRates::new().map_ok(Some);
					let (contact_information, employer) = futures::try_join!(
						CAdapter::retrieve(&connection, &match_all_contacts).map_ok(|mut vec| {
							vec.sort_by(|lhs, rhs| lhs.label.cmp(&rhs.label));
							vec
						}),
						OAdapter::retrieve(&connection, &match_employer)
							.and_then(|mut vec| future::ready(vec.pop().ok_or(sqlx::Error::RowNotFound))),
					)?;

					let exchange_rates = exchange_rates_fut.await?;
					let selected = input::select(&retrieved, "Select the Jobs to export")?;

					#[rustfmt::skip]
					stream::iter(selected.into_iter().map(Ok)).try_for_each_concurrent(None, |j| {
						let connection = &connection;
						let contact_information = &contact_information;
						let employer = &employer;
						let exchange_rates = exchange_rates.as_ref();
						let match_condition = MatchTimesheet { job: j.id.into(), ..Default::default() };
						let output_dir = output_dir.as_ref();

						async move {
							let timesheets_fut = TAdapter::retrieve(connection, &match_condition)
								.map_ok(|mut v| {
									v.sort_by(|lhs, rhs| lhs.time_begin.cmp(&rhs.time_begin));
									v
								});

							let filename =
								format!("{}--{}.{}", j.client.name.replace(' ', "-"), j.id, format.extension());

							let timesheets = timesheets_fut.await?;

							let exported =
								format.export_job(&j, contact_information, exchange_rates, employer, &timesheets);

							match output_dir
							{
								Some(d) => fs::write(d.join(filename), exported).await,
								_ => fs::write(filename, exported).await,
							}?;

							DynResult::Ok(())
						}
					})
					.await?;
				}
			},

			RetrieveCommand::Location =>
			{
				retrieve::<LAdapter, _, _>(&connection, self.match_args, true).await?;
			},

			RetrieveCommand::Organization {
				employer,
				set_employer,
			} =>
			{
				let match_condition = match employer
				{
					false => self.match_args.try_into()?,
					_ => config
						.organizations
						.employer_id
						.ok_or_else(|| Error::NotConfigured("employer_id".into(), "organizations".into()))
						.map(|id| Some(id.into()))?,
				};

				let retrieved =
					retrieve::<OAdapter, _, _>(&connection, match_condition, !set_employer).await?;

				if set_employer
				{
					let selected = input::select_one(
						&retrieved,
						"Select the Employer to use in your configuration",
					)?;
					let mut c = config;
					c.organizations.employer_id = Some(selected.id);
					c.write()?;
				}
			},

			RetrieveCommand::Timesheet =>
			{
				retrieve::<TAdapter, _, _>(&connection, self.match_args, true).await?;
			},
		};

		Ok(())
	}
}
