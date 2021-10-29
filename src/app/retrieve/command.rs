use core::fmt::Display;
use std::borrow::Cow::Owned;

use clinvoice_adapter::data::{Deletable, EmployeeAdapter, JobAdapter, LocationAdapter, OrganizationAdapter, PersonAdapter, TimesheetAdapter, Updatable};
use clinvoice_config::Config;
use clinvoice_query as query;
use clinvoice_data::{chrono::Utc, views::RestorableSerde, Location};
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

		#[structopt(help = "Export retrieved entities to markdown", long, short)]
		export: bool,

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

	#[structopt(about = "Retrieve existing records about people")]
	Person,
}

impl Command
{
	/// # Summary
	///
	/// Delete some `entities`
	///
	/// `delete_entity` determines how the entities are deleted.
	async fn delete<'err, D, Db, Entity, EntityView>(
		connection: &Pool<Db>,
		cascade: bool,
		entities: &[EntityView],
	) -> DynResult<'err, ()>
	where
		D: Deletable<Db = Db, Entity = Entity>,
		Db: Database,
		EntityView: Clone + Display + Into<Entity> + Send,
		for<'c> &'c mut Db::Connection: Executor<'c, Database = Db>,
	{
		let selection = input::select(entities, "Select the entities you want to delete")?;
		D::delete(
			connection,
			cascade,
			selection.into_iter().map(EntityView::into),
		)
		.await?;
		Ok(())
	}

	/// # Summary
	///
	/// Edit some `entities`, and then update them.
	///
	/// `update_entity` determines how the entities are updated.
	async fn update<'err, Db, Entity, EntityView, U>(
		connection: &Pool<Db>,
		entities: &[EntityView],
	) -> DynResult<'err, ()>
	where
		Db: Database,
		EntityView:
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

				v.push(U::update(connection, edited.into()));
				Ok(v)
			})?;

		future::try_join_all(updates).await?;
		Ok(())
	}

	pub async fn run<'err, Db, EAdapter, JAdapter, LAdapter, OAdapter, PAdapter, TAdapter>(
		self,
		connection: Pool<Db>,
		cascade_delete: bool,
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
		PAdapter: Deletable<Db = Db> + PersonAdapter + Send,
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
				let results_view = input::util::employee::retrieve_view::<&str, _, EAdapter>(
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
					Self::delete::<EAdapter, _, _, _>(&connection, cascade_delete, &results_view)
						.await?;
				}

				if update
				{
					Self::update::<_, _, _, EAdapter>(&connection, &results_view).await?
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
					results_view.into_iter().for_each(|e| println!("{}", e));
				}
			},

			Self::Job {
				close,
				export,
				reopen,
			} =>
			{
				let results_view = input::util::job::retrieve_view::<&str, _, JAdapter>(
					&connection,
					"Query the `Job` you are looking for",
					false,
				)
				.await?;

				if delete
				{
					Self::delete::<JAdapter, _, _, _>(&connection, cascade_delete, &results_view)
						.await?;
				}

				if update
				{
					Self::update::<_, _, _, JAdapter>(&connection, &results_view).await?
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
							JAdapter::update(&connection, j.into()).await
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
							JAdapter::update(&connection, j.into()).await
						})
						.await?;
				}

				if export
				{
					let to_export =
						input::select(&results_view, "Select which Jobs you want to export")?;

					// WARN: this `let` seems redundant, but the "type needs to be known at this point"
					let export_result: DynResult<'_, _> = stream::iter(to_export.into_iter().map(Ok))
						.try_for_each_concurrent(None, |job| async move {
							let timesheets = TAdapter::retrieve_view(&connection, &query::Timesheet {
								job: query::Job {
									id: query::Match::EqualTo(Owned(job.id)),
									..Default::default()
								},
								..Default::default()
							}).await?;
							let export = job.export(&timesheets)?;
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
					results_view.iter().for_each(|j| println!("{}", j));
				}
			},

			Self::Location { create_inner } =>
			{
				let results_view = input::util::location::retrieve_view::<&str, _, LAdapter>(
					&connection,
					"Query the `Location` you are looking for",
					false,
				)
				.await?;

				if delete
				{
					Self::delete::<LAdapter, _, _, _>(&connection, cascade_delete, &results_view)
						.await?;
				}

				if update
				{
					Self::update::<_, _, _, LAdapter>(&connection, &results_view).await?
				}

				if let Some(name) = create_inner.last()
				{
					let location = input::select_one(
						&results_view,
						format!("Select the outer Location of {}", name),
					)?;
					stream::iter(create_inner.into_iter().map(Ok).rev())
						.try_fold(location.into(), |loc: Location, name: String| async {
							LAdapter::create_inner(&connection, &loc.into(), name).await
						})
						.await?;
				}
				else if !(delete || update)
				{
					results_view.iter().for_each(|l| println!("{}", l));
				}
			},

			Self::Organization =>
			{
				let results_view = input::util::organization::retrieve_view::<&str, _, OAdapter>(
					&connection,
					"Query the `Organization` you are looking for",
					false,
				)
				.await?;

				if delete
				{
					Self::delete::<OAdapter, _, _, _>(&connection, cascade_delete, &results_view)
						.await?;
				}

				if update
				{
					Self::update::<_, _, _, OAdapter>(&connection, &results_view).await?
				}
				else if !delete
				{
					results_view.iter().for_each(|o| println!("{}", o));
				}
			},

			Self::Person =>
			{
				let results_view = input::util::person::retrieve_view::<&str, _, PAdapter>(
					&connection,
					"Query the `Person` you are looking for",
					false,
				)
				.await?;

				if delete
				{
					Self::delete::<PAdapter, _, _, _>(&connection, cascade_delete, &results_view)
						.await?;
				}

				if update
				{
					Self::update::<_, _, _, PAdapter>(&connection, &results_view).await?
				}
				else if !delete
				{
					results_view.iter().for_each(|p| println!("{}", p));
				}
			},
		};

		Ok(())
	}
}
