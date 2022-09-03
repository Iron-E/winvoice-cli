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
use clinvoice_config::Config;
use clinvoice_match::{MatchOrganization, MatchTimesheet};
use clinvoice_schema::{chrono::Utc, InvoiceDate};
use futures::{future, stream, TryFutureExt, TryStreamExt};
use money2::{Exchange, ExchangeRates};
use serde::{de::DeserializeOwned, Serialize};
use sqlx::{Database, Executor, Pool};
use tokio::fs;

use super::{Retrieve, RetrieveCommand};
use crate::{args::RunAction, fmt, input, DynResult};

#[async_trait::async_trait(?Send)]
impl RunAction for Retrieve
{
	async fn action<CAdapter, EAdapter, JAdapter, LAdapter, OAdapter, TAdapter, XAdapter, Db>(
		self,
		connection: Pool<Db>,
		config: Config,
	) -> DynResult<()>
	where
		Db: Database,
		CAdapter: Deletable<Db = Db> + ContactAdapter,
		EAdapter: Deletable<Db = Db> + EmployeeAdapter,
		JAdapter: Deletable<Db = Db> + JobAdapter,
		LAdapter: Deletable<Db = Db> + LocationAdapter,
		OAdapter: Deletable<Db = Db> + OrganizationAdapter,
		TAdapter: Deletable<Db = Db> + TimesheetAdapter,
		XAdapter: Deletable<Db = Db> + ExpensesAdapter,
		for<'connection> &'connection mut Db::Connection: Executor<'connection, Database = Db>,
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
		/// function, as they all implement `Retr` at the minimum.
		async fn retrieve<Retr, Db, Match>(
			connection: &Pool<Db>,
			match_condition: Match,
			print: bool,
		) -> DynResult<Vec<Retr::Entity>>
		where
			Db: Database,
			Match: TryInto<Option<Retr::Match>>,
			Match::Error: 'static + StdError,
			Retr: Retrievable<Db = Db>,
			Retr::Entity: Clone + Display + Sync,
			Retr::Match: Default + DeserializeOwned + Serialize,
			for<'connection> &'connection mut Db::Connection: Executor<'connection, Database = Db>,
		{
			let retrieved = match match_condition.try_into()?
			{
				Some(condition) => Retr::retrieve(connection, condition).await?,

				#[rustfmt::skip]
				None => input::retrieve::<Retr, _, _>(
					connection,
					format!("Query the {} to delete", fmt::type_name::<Retr::Entity>()),
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

			RetrieveCommand::Employee { default, set_default } =>
			{
				let match_condition = match default
				{
					false => self.match_args.try_into()?,
					true => config.employees.id_or_err().map(|id| Some(id.into()))?,
				};

				let retrieved =
					retrieve::<EAdapter, _, _>(&connection, match_condition, !set_default).await?;

				if set_default
				{
					let selected =
						input::select_one(retrieved, "Select the Employee to set as the default")?;
					let mut c = config;
					c.employees.id = Some(selected.id);
					c.write()?;
				}
			},

			RetrieveCommand::Expense =>
			{
				retrieve::<XAdapter, _, _>(&connection, self.match_args, true).await?;
			},

			RetrieveCommand::Job { currency, export, output_dir } =>
			{
				let retrieved =
					retrieve::<JAdapter, _, _>(&connection, self.match_args, export.is_none())
						.await?;

				if let Some(format) = export
				{
					let match_all_contacts = Default::default();
					let match_employer =
						config.organizations.employer_id_or_err().map(MatchOrganization::from)?;

					let exchange_rates_fut = ExchangeRates::new().map_ok(Some);
					let (contact_information, employer) = futures::try_join!(
						CAdapter::retrieve(&connection, match_all_contacts).map_ok(|mut vec| {
							vec.sort_by(|lhs, rhs| lhs.label.cmp(&rhs.label));
							vec
						}),
						OAdapter::retrieve(&connection, match_employer).and_then(|mut vec| {
							future::ready(vec.pop().ok_or(sqlx::Error::RowNotFound))
						}),
					)?;

					let exchange_rates = exchange_rates_fut.await?;
					let mut selected = input::select(retrieved, "Select the Jobs to export")?;

					selected
						.iter_mut()
						.filter(|j| j.invoice.date.and_then(|d| d.paid).is_none())
						.for_each(|j| {
							j.invoice.date = Some(InvoiceDate { issued: Utc::now(), paid: None });
						});

					#[rustfmt::skip]
					stream::iter(selected.into_iter().map(Ok)).try_for_each_concurrent(None, |j| {
						let connection = &connection;
						let contact_information = &contact_information;
						let employer = &employer;
						let exchange_rates = exchange_rates.as_ref();
						let match_condition = MatchTimesheet { job: j.id.into(), ..Default::default() };
						let output_dir = output_dir.as_ref();

						async move {
							let timesheets_fut = TAdapter::retrieve(connection, match_condition)
								.map_ok(|mut v| {
									v.sort_by(|lhs, rhs| lhs.time_begin.cmp(&rhs.time_begin));
									v
								});

							let filename =
								format!("{}--{}.{}", j.client.name.replace(' ', "-"), j.id, format.extension());

							let timesheets = timesheets_fut.await?;

							let exported = format.export_job(
								&match exchange_rates
								{
									Some(r) => j.exchange(currency, r),
									None => j,
								},
								contact_information,
								employer,
								&timesheets,
							);

							match output_dir
							{
								Some(d) => fs::write(d.join(filename), exported).await,
								None => fs::write(filename, exported).await,
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

			RetrieveCommand::Organization { employer, set_employer } =>
			{
				let match_condition = match employer
				{
					false => self.match_args.try_into()?,
					true => config.organizations.employer_id_or_err().map(|id| Some(id.into()))?,
				};

				let retrieved =
					retrieve::<OAdapter, _, _>(&connection, match_condition, !set_employer).await?;

				if set_employer
				{
					let selected = input::select_one(
						retrieved,
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
