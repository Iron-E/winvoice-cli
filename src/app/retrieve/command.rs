use core::fmt::Display;

use clinvoice_adapter::{
	schema::{EmployeeAdapter, JobAdapter, LocationAdapter, OrganizationAdapter, TimesheetAdapter},
	Deletable,
	Updatable,
};
use clinvoice_config::Config;
use clinvoice_finance::ExchangeRates;
use clinvoice_match::{MatchJob, MatchTimesheet};
use clinvoice_schema::{chrono::Utc, Currency, Location, RestorableSerde};
use futures::{
	future,
	stream::{self, TryStreamExt},
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
	async fn delete<'err, D, Db, Entity>(
		connection: &Pool<Db>,
		entities: &[Entity],
	) -> DynResult<'err, ()>
	where
		D: Deletable<Db = Db, Entity = Entity>,
		Db: Database,
		Entity: Clone + Display + Send,
		for<'c> &'c mut Db::Connection: Executor<'c, Database = Db>,
	{
		let selection = input::select(entities, "Select the entities you want to delete")?;
		D::delete(connection, selection.into_iter()).await?;
		Ok(())
	}

	/// # Summary
	///
	/// Edit some `entities`, and then update them.
	///
	/// `update_entity` determines how the entities are updated.
	async fn update<'err, Db, Entity, U>(
		connection: &Pool<Db>,
		entities: &[Entity],
	) -> DynResult<'err, ()>
	where
		Db: Database,
		Entity:
			Clone + DeserializeOwned + Display + Into<Entity> + RestorableSerde + Serialize + Send,
		U: Updatable<Db = Db, Entity = Entity>,
		for<'c> &'c mut Db::Connection: Executor<'c, Database = Db>,
	{
		let selection = input::select(entities, "Select the entities you want to update")?;

		// PERF: all of the `update_entity` operations are queued in the background while the user keeps editing. this is in case users have slow internet connection
		let updates = selection
			.into_iter()
			.try_fold(Vec::new(), |mut v, entity| {
				let edited = match input::edit_and_restore(&entity, "Make any desired edits")
				{
					Ok(e) => e,
					Err(input::Error::NotEdited) => entity,
					Err(e) => return Err(e),
				};

				v.push(U::update(connection, edited));
				Ok(v)
			})?;

		future::try_join_all(updates).await?;
		Ok(())
	}

	pub async fn run<'err, Db, EAdapter, JAdapter, LAdapter, OAdapter, TAdapter>(
		self,
		connection: Pool<Db>,
		config: &Config<'_, '_>,
		delete: bool,
		update: bool,
	) -> DynResult<'err, ()>
	where
		Db: Database,
		EAdapter: Deletable<Db = Db> + EmployeeAdapter + Send,
		JAdapter: Deletable<Db = Db> + JobAdapter + Send,
		LAdapter: Deletable<Db = Db> + LocationAdapter + Send,
		OAdapter: Deletable<Db = Db> + OrganizationAdapter + Send,
		TAdapter: Deletable<Db = Db> + TimesheetAdapter + Send,
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
					if default
					{
						config.employees.default_id
					}
					else
					{
						None
					},
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
					new_config.employees.default_id = Some(
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

					new_config.update()?;
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
					let unclosed: Vec<_> = results_view
						.iter()
						.filter(|j| j.date_close.is_none())
						.cloned()
						.collect();
					let selected = input::select(&unclosed, "Select the Jobs you want to close")?;
					stream::iter(selected.into_iter().map(Ok))
						.try_for_each_concurrent(None, |mut j| async {
							j.date_close = Some(Utc::now());
							JAdapter::update(&connection, j).await
						})
						.await?;
				}

				if reopen
				{
					let closed: Vec<_> = results_view
						.iter()
						.filter(|j| j.date_close.is_some())
						.cloned()
						.collect();
					let selected = input::select(&closed, "Select the Jobs you want to reopen")?;
					stream::iter(selected.into_iter().map(Ok))
						.try_for_each_concurrent(None, |mut j| async {
							j.date_close = None;
							JAdapter::update(&connection, j).await
						})
						.await?;
				}

				if let Some(e) = export
				{
					let exchange_rates = if e == Default::default()
					{
						None
					}
					else
					{
						Some(ExchangeRates::new())
					};

					let to_export =
						input::select(&results_view, "Select which Jobs you want to export")?;

					let exchange_rates = match exchange_rates
					{
						Some(fut) => Some(fut.await?),
						_ => None,
					};

					let conn = &connection;
					let exchange_rates = exchange_rates.as_ref();
					// WARN: this `let` seems redundant, but the "type needs to be known at this point"
					let export_result: DynResult<'_, _> = stream::iter(to_export.into_iter().map(Ok))
						.try_for_each_concurrent(None, |job| async move {
							let timesheets = TAdapter::retrieve(conn, MatchTimesheet {
								job: MatchJob {
									id: job.id.into(),
									..Default::default()
								},
								..Default::default()
							})
							.await?;

							let export = job.export(exchange_rates, &timesheets)?;
							fs::write(
								format!("{}--{}.md", job.client.name.replace(' ', "-"), job.id),
								export,
							)
							.await?;
							Ok(())
						})
						.await;
					export_result?;
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

					let conn = &connection;
					stream::iter(create_inner.into_iter().map(Ok).rev())
						.try_fold(location, |loc: Location, name: String| async move {
							LAdapter::create_inner(conn, loc, name).await
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
