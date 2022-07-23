mod command;

use core::fmt::Display;
use std::error::Error;

use clap::Args as Clap;
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
use clinvoice_config::{Adapters, Config, Error as ConfigError};
use clinvoice_finance::ExchangeRates;
use clinvoice_match::{MatchEmployee, MatchOrganization, MatchTimesheet};
use command::RetrieveCommand;
use futures::{stream, TryFutureExt, TryStreamExt};
use serde::{de::DeserializeOwned, Serialize};
use sqlx::{Database, Executor, Pool};
use tokio::fs;

use super::{match_args::MatchArgs, store_args::StoreArgs};
use crate::{fmt, input, DynResult};

/// Retrieve information being stored by CLInvoice.
#[derive(Clap, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Retrieve
{
	/// The specific object to [`Retrieve`] and related arguments.
	#[clap(subcommand)]
	command: RetrieveCommand,

	/// Specifies a file which can be used in place of the prompt of a user query.
	#[clap(flatten)]
	match_args: MatchArgs,

	/// Specifies the [`Store`](clinvoice_config::Store) to [`Retrieve`] from.
	#[clap(flatten)]
	store_args: StoreArgs,
}

impl Retrieve
{
	/// [`Retrieve`] an entity according to the [`RetrieveCommand`].
	///
	/// The [`StoreArgs`] must be resolved into a `connection` by this point.
	async fn retrieve<CAdapter, EAdapter, JAdapter, LAdapter, OAdapter, TAdapter, XAdapter, TDb>(
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
			TMatch::Error: 'static + Error,
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
				let match_condition = self.match_args.try_into().and_then(|condition| {
					default
						.then(|| {
							config.employees.id.map(MatchEmployee::from).ok_or_else(|| {
								ConfigError::NotConfigured("id".into(), "employees".into()).into()
							})
						})
						.transpose()
						.map(|default_condition| default_condition.or(condition))
				})?;

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
					let employer_id = config.organizations.employer_id.ok_or_else(|| {
						ConfigError::NotConfigured("employer_id".into(), "organizations".into())
					})?;
					let exchange_rates_fut = ExchangeRates::new().map_ok(Some);
					let match_all_contacts = Default::default();
					let selected = input::select(&retrieved, "Select the Jobs to export")?;

					let contact_information_fut = CAdapter::retrieve(&connection, &match_all_contacts)
						.map_ok(|mut vec| {
							vec.sort_by(|lhs, rhs| lhs.label.cmp(&rhs.label));
							vec
						});

					let employer = OAdapter::retrieve(&connection, &employer_id.into())
						.await
						.and_then(|mut vec| vec.pop().ok_or(sqlx::Error::RowNotFound))?;

					let contact_information = contact_information_fut.await?;
					let exchange_rates = exchange_rates_fut.await?;

					stream::iter(selected.into_iter().map(Ok))
						.try_for_each_concurrent(None, |job| {
							let connection = &connection;
							let contact_information = &contact_information;
							let employer = &employer;
							let exchange_rates = exchange_rates.as_ref();
							let output_dir = output_dir.as_ref();

							async move {
								let timesheets = {
									let mut t = TAdapter::retrieve(connection, &MatchTimesheet {
										job: job.id.into(),
										..Default::default()
									})
									.await?;

									t.sort_by(|lhs, rhs| lhs.time_begin.cmp(&rhs.time_begin));
									t
								};

								let exported = format.export_job(
									&job,
									contact_information,
									exchange_rates,
									employer,
									&timesheets,
								);

								let filename = format!(
									"{}--{}.{}",
									job.client.name.replace(' ', "-"),
									job.id,
									format.extension(),
								);

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
				let match_condition = self.match_args.try_into().and_then(|condition| {
					employer
						.then(|| {
							config
								.organizations
								.employer_id
								.map(MatchOrganization::from)
								.ok_or_else(|| {
									ConfigError::NotConfigured("employer_id".into(), "organizations".into())
										.into()
								})
						})
						.transpose()
						.map(|employer_condition| employer_condition.or(condition))
				})?;

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

	pub async fn run(self, config: Config) -> DynResult<()>
	{
		let store = self.store_args.try_get_from(&config)?;

		match store.adapter
		{
			#[cfg(feature = "postgres")]
			Adapters::Postgres =>
			{
				use clinvoice_adapter_postgres::schema::{
					PgContact,
					PgEmployee,
					PgExpenses,
					PgJob,
					PgLocation,
					PgOrganization,
					PgTimesheet,
				};

				let pool = Pool::connect_lazy(&store.url)?;
				self
					.retrieve::<PgContact, PgEmployee, PgJob, PgLocation, PgOrganization, PgTimesheet, PgExpenses, _>(
						pool, config,
					)
					.await?
			},

			// NOTE: this is allowed because there may be additional adapters added later, and I want
			//       to define this behavior now.
			#[allow(unreachable_patterns)]
			_ => return Err(ConfigError::FeatureNotFound(store.adapter).into()),
		};

		Ok(())
	}
}
