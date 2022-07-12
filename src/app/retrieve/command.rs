use core::fmt::Display;
use std::error::Error;

use clinvoice_adapter::{
	schema::{
		ContactInfoAdapter,
		EmployeeAdapter,
		JobAdapter,
		LocationAdapter,
		OrganizationAdapter,
		TimesheetAdapter,
	},
	Deletable,
	Updatable,
};
use clinvoice_config::Config;
use clinvoice_export::Format;
use clinvoice_finance::ExchangeRates;
use clinvoice_match::{MatchJob, MatchOrganization, MatchTimesheet};
use clinvoice_schema::{chrono::Utc, Currency, Location, RestorableSerde};
use futures::{
	stream::{self, TryStreamExt},
	TryFutureExt,
};
use serde::{de::DeserializeOwned, Serialize};
use sqlx::{Database, Executor, Pool};
use structopt::StructOpt;
use tokio::fs;

use crate::{input, DynResult};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructOpt)]
pub enum Command
{
	#[structopt(about = "Retrieve existing records about employees")]
	Employee
	{
		#[structopt(
			help = "Retrieve the default employee as specified in your configuration",
			long,
			short
		)]
		default: bool,

		#[structopt(
			help = "Set one of the employees as the default in your configuration",
			long,
			short
		)]
		set_default: bool,
	},

	#[structopt(about = "Retrieve existing records about job")]
	Job
	{
		#[structopt(help = "Select jobs to be closed", long, short)]
		close: bool,

		#[structopt(
			help = "Export retrieved entities to markdown using the specified currency",
			long,
			short
		)]
		export: Option<Currency>,

		#[structopt(help = "Select jobs to be reopened", long, short)]
		reopen: bool,
	},

	#[structopt(about = "Retrieve existing records about locations")]
	Location
	{
		#[structopt(
			help = "Create a new location inside of some selected location\nArgument is the same as \
			        `clinvoice create location`",
			long,
			short
		)]
		create_inner: Vec<String>,
	},

	#[structopt(about = "Retrieve existing records about organizations")]
	Organization,
}

impl Command
{
	/// # Summary
	///
	/// Delete some `entities`
	///
	/// `delete_entity` determines how the entities are deleted.
	async fn delete<D, Db, Entity>(connection: &Pool<Db>, entities: &[Entity]) -> DynResult<()>
	where
		D: Deletable<Db = Db, Entity = Entity>,
		Db: Database,
		Entity: Clone + Display + Sync,
		for<'c> &'c mut Db::Connection: Executor<'c, Database = Db>,
	{
		let selection = input::select(entities, "Select the entities you want to delete")?;
		D::delete(connection, selection.iter()).await?;
		Ok(())
	}

	/// # Summary
	///
	/// Edit some `entities`, and then update them.
	///
	/// `update_entity` determines how the entities are updated.
	async fn update<Db, Entity, U>(connection: &Pool<Db>, entities: &[Entity]) -> DynResult<()>
	where
		Db: Database,
		Entity: Clone + DeserializeOwned + Display + RestorableSerde + Serialize + Sync,
		U: Updatable<Db = Db, Entity = Entity>,
		for<'c> &'c mut Db::Connection: Executor<'c, Database = Db>,
	{
		let selection = input::select(entities, "Select the entities you want to update")?;

		let edits = selection
			.into_iter()
			.filter_map(
				|entity| match input::edit_and_restore(&entity, "Make any desired edits")
				{
					Err(input::Error::NotEdited) => None,
					result => Some(result),
				},
			)
			.collect::<input::Result<Vec<_>>>()?;

		connection
			.begin()
			.and_then(|mut transaction| async {
				U::update(&mut transaction, edits.iter()).await?;
				transaction.commit().await
			})
			.await?;

		Ok(())
	}

	pub async fn run<Db, CAdapter, EAdapter, JAdapter, LAdapter, OAdapter, TAdapter>(
		self,
		connection: Pool<Db>,
		config: &Config,
		delete: bool,
		update: bool,
	) -> DynResult<()>
	where
		Db: Database,
		CAdapter: Deletable<Db = Db> + ContactInfoAdapter,
		EAdapter: Deletable<Db = Db> + EmployeeAdapter,
		JAdapter: Deletable<Db = Db> + JobAdapter,
		LAdapter: Deletable<Db = Db> + LocationAdapter,
		OAdapter: Deletable<Db = Db> + OrganizationAdapter,
		TAdapter: Deletable<Db = Db> + TimesheetAdapter,
		for<'c> &'c mut Db::Connection: Executor<'c, Database = Db>,
	{
		match self
		{
			Self::Employee {
				default,
				set_default,
			} =>
			{
				let results_view = input::util::employee::retrieve::<&str, _, EAdapter>(
					&connection,
					if default { config.employees.id } else { None },
					"Query the `Employee` you are looking for",
					false,
				)
				.await?;

				if delete
				{
					Self::delete::<EAdapter, _, _>(&connection, &results_view).await?;
				}

				if update
				{
					Self::update::<_, _, EAdapter>(&connection, &results_view).await?
				}

				if set_default
				{
					let mut new_config = config.clone();
					new_config.employees.id = Some(
						if results_view.len() > 1
						{
							input::select_one(&results_view, "Which `Employee` should be the default?")?.id
						}
						else
						{
							results_view
								.first()
								.ok_or_else(|| input::Error::NoData("`Employee`".into()))?
								.id
						},
					);

					new_config.write()?;
				}
				else if !(delete || update)
				{
					results_view.into_iter().for_each(|e| println!("{e}"));
				}
			},

			Self::Job {
				close,
				export,
				reopen,
			} =>
			{
				let results_view = input::util::job::retrieve::<&str, _, JAdapter>(
					&connection,
					"Query the `Job` you are looking for",
					false,
				)
				.await?;

				if delete
				{
					Self::delete::<JAdapter, _, _>(&connection, &results_view).await?;
				}

				if update
				{
					Self::update::<_, _, JAdapter>(&connection, &results_view).await?
				}

				if close
				{
					let mut selected = input::select(
						results_view
							.iter()
							.filter(|j| j.date_close.is_none())
							.cloned()
							.collect::<Vec<_>>()
							.as_slice(),
						"Select the Jobs you want to close",
					)?;

					let transaction_fut = connection.begin();

					let now = Some(Utc::now());
					selected.iter_mut().for_each(|j| j.date_close = now);

					transaction_fut
						.and_then(|mut transaction| async {
							JAdapter::update(&mut transaction, selected.iter()).await?;
							transaction.commit().await
						})
						.await?;
				}

				if reopen
				{
					let mut selected = input::select(
						results_view
							.iter()
							.filter(|j| j.date_close.is_some())
							.cloned()
							.collect::<Vec<_>>()
							.as_slice(),
						"Select the Jobs you want to reopen",
					)?;

					let transaction_fut = connection.begin();

					selected.iter_mut().for_each(|j| j.date_close = None);

					transaction_fut
						.and_then(|mut transaction| async {
							JAdapter::update(&mut transaction, selected.iter()).await?;
							transaction.commit().await
						})
						.await?;
				}

				if let Some(e) = export
				{
					let exchange_rates_fut = async {
						if e == Default::default()
						{
							Ok(None)
						}
						else
						{
							ExchangeRates::new().await.map(Some)
						}
					};

					let match_all_contacts = Default::default();
					let contact_information_fut = CAdapter::retrieve(&connection, &match_all_contacts)
						.map_ok(|mut vec| {
							vec.sort_by(|lhs, rhs| lhs.label.cmp(&rhs.label));
							vec
						});

					let employer_id = config.organizations.employer_id.ok_or_else(|| {
						"You must specify the `Organization` you work for before exporting `Job`s."
							.to_string()
					})?;
					let employer = OAdapter::retrieve(&connection, &MatchOrganization {
						id: employer_id.into(),
						..Default::default()
					})
					.err_into::<Box<dyn Error>>()
					.await
					.and_then(|mut vec| {
						vec.pop().ok_or_else(|| {
							format!(
								"Your configuration specifies that your employer has ID {employer_id}, \
								 however no `Organization` in the database has this ID."
							)
							.into()
						})
					})?;

					let contact_information = contact_information_fut.await?;
					let exchange_rates = exchange_rates_fut.await?;

					let to_export =
						input::select(&results_view, "Select which Jobs you want to export")?;

					stream::iter(to_export.into_iter().map(Ok))
						.try_for_each_concurrent(None, |job| {
							let conn = &connection;
							let exchange_rates_ref = exchange_rates.as_ref();
							let org = &employer;
							let contacts = &contact_information;

							async move {
								let mut timesheets = TAdapter::retrieve(conn, &MatchTimesheet {
									job: MatchJob {
										id: job.id.into(),
										..Default::default()
									},
									..Default::default()
								})
								.await?;
								timesheets.sort_by(|lhs, rhs| lhs.time_begin.cmp(&rhs.time_begin));

								fs::write(
									format!("{}--{}.md", job.client.name.replace(' ', "-"), job.id),
									Format::Markdown.export_job(
										&job,
										contacts,
										exchange_rates_ref,
										org,
										&timesheets,
									),
								)
								.await?;

								DynResult::Ok(())
							}
						})
						.await?;
				}
				else if !(close || delete || reopen || update)
				{
					results_view.iter().for_each(|j| println!("{j}"));
				}
			},

			Self::Location { create_inner } =>
			{
				let results_view = input::util::location::retrieve::<&str, _, LAdapter>(
					&connection,
					"Query the `Location` you are looking for",
					false,
				)
				.await?;

				if delete
				{
					Self::delete::<LAdapter, _, _>(&connection, &results_view).await?;
				}

				if update
				{
					Self::update::<_, _, LAdapter>(&connection, &results_view).await?
				}

				if let Some(name) = create_inner.last()
				{
					let location = input::select_one(
						&results_view,
						format!("Select the outer Location of {name}"),
					)?;

					stream::iter(create_inner.into_iter().map(Ok).rev())
						.try_fold(location, |loc: Location, name: String| {
							LAdapter::create(&connection, name, Some(loc))
						})
						.await?;
				}
				else if !(delete || update)
				{
					results_view.iter().for_each(|l| println!("{l}"));
				}
			},

			Self::Organization =>
			{
				let results_view = input::util::organization::retrieve::<&str, _, OAdapter>(
					&connection,
					"Query the `Organization` you are looking for",
					false,
				)
				.await?;

				if delete
				{
					Self::delete::<OAdapter, _, _>(&connection, &results_view).await?;
				}

				if update
				{
					Self::update::<_, _, OAdapter>(&connection, &results_view).await?
				}
				else if !delete
				{
					results_view.iter().for_each(|o| println!("{o}"));
				}
			},
		};

		Ok(())
	}
}
