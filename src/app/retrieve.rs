use core::fmt::Display;
use std::error::Error;

use clinvoice_adapter::{Adapters, Error as AdapterError, Store, data::{Deletable, LocationAdapter, Updatable}};
use clinvoice_data::{chrono::Utc, views::RestorableSerde, Location};
use clinvoice_serialize::markdown;
use futures::{
	future,
	stream::{self, TryStreamExt},
	Future,
	TryFutureExt,
};
use serde::{de::DeserializeOwned, Serialize};
use tokio::fs;

use crate::{input, Config, DynResult, StructOpt};

#[cfg(feature="postgres")]
use clinvoice_adapter_postgres::data::{PostgresEmployee, PostgresJob, PostgresLocation, PostgresOrganization, PostgresPerson};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructOpt)]
#[structopt(about = "Retrieve information that was recorded with CLInvoice")]
pub(super) struct Retrieve
{
	#[structopt(help = "Select retrieved entities for deletion. See -c", long, short)]
	pub delete: bool,

	#[structopt(
		help = "Cascade -d operations. Without this flag, entities referenced by other entities \
		        cannot be deleted",
		long,
		short
	)]
	pub cascade: bool,

	#[structopt(help = "Select retrieved entities for data updating", long, short)]
	pub update: bool,

	#[structopt(subcommand)]
	pub command: RetrieveCommand,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructOpt)]
pub(super) enum RetrieveCommand
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
			help = "Export retrieved entities to markdown",
			long,
			short
		)]
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

impl Retrieve
{
	/// # Summary
	///
	/// Delete some `entities`
	///
	/// `delete_entity` determines how the entities are deleted.
	async fn delete<'err, E, F, Fut, T>(entities: &[T], delete_entity: F) -> DynResult<'err, ()>
	where
		E: Error + 'err,
		F: Fn(T) -> Fut,
		Fut: Future<Output = Result<(), E>>,
		T: Clone + Display,
	{
		let selection = input::select(entities, "Select the entities you want to delete")?;
		stream::iter(selection.into_iter().map(Ok))
			.try_for_each_concurrent(None, |entity| async { delete_entity(entity).await })
			.err_into()
			.await
	}

	/// # Summary
	///
	/// Edit some `entities`, and then update them.
	///
	/// `update_entity` determines how the entities are updated.
	async fn update<'err, E, F, Fut, T>(entities: &[T], update_entity: F) -> DynResult<'err, ()>
	where
		E: Error + 'err,
		F: Fn(T) -> Fut,
		Fut: Future<Output = Result<(), E>>,
		T: Clone + DeserializeOwned + Display + RestorableSerde + Serialize,
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

				v.push(update_entity(edited));
				Ok(v)
			})?;

		future::try_join_all(updates).await?;
		Ok(())
	}

	/// # Summary
	///
	/// Execute the constructed command.
	pub(super) async fn run<'err>(
		self,
		config: Config<'_, '_>,
		store: &Store,
	) -> DynResult<'err, ()>
	{
		let adapter_not_enabled =
			|| -> DynResult<'err, ()> { Err(AdapterError::FeatureNotFound(store.adapter).into()) };

		match self.command
		{
			RetrieveCommand::Employee {
				default,
				set_default,
			} =>
			{
				macro_rules! retrieve {
					($Emp:ident, $pool:ident) => {{
						let results_view =
							input::util::employee::retrieve_view::<&str, $Emp, _>(
								if default
								{
									Some(config.employees.default_id)
								}
								else
								{
									None
								},
								"Query the `Employee` you are looking for",
								false,
								pool,
							)
							.await?;

						if self.delete
						{
							Self::delete(&results_view, |e| async {
								$Emp {
									employee: &(e.into()),
									store,
								}
								.delete(self.cascade)
								.await
							})
							.await?;
						}

						if self.update
						{
							Self::update(&results_view, |e| async {
								$Emp {
									employee: &(e.into()),
									store,
								}
								.update()
								.await
							})
							.await?;
						}

						if set_default
						{
							let mut new_config = config.clone();
							new_config.employees.default_id = match results_view.len() > 1
							{
								false =>
								{
									results_view
										.first()
										.ok_or_else(|| {
											input::Error::NoData(format!("`{}`", stringify!(Employee)))
										})?
										.id
								},
								_ =>
								{
									input::select_one(
										&results_view,
										"Which `Employee` should be the default?",
									)?
									.id
								},
							};

							new_config.update().await?;
						}
						else if !(self.delete || self.update)
						{
							results_view.iter().for_each(|e| println!("{}", e));
						}
					}};
				}

				match store.adapter
				{
					#[cfg(feature="postgres")]
					Adapters::Postgres => retrieve!(PostgresEmployee, pool),

					_ => return adapter_not_enabled(),
				};
			},

			RetrieveCommand::Job {
				close,
				export,
				reopen,
			} =>
			{
				macro_rules! retrieve {
					($Emp:ident, $job:ident, $loc:ident, $org:ident, $per:ident) => {{
						let results_view =
							input::util::job::retrieve_view::<&str, $Emp, $job, $loc, $org, $per>(
								"Query the `Job` you are looking for",
								false,
								store,
							)
							.await?;

						if self.delete
						{
							Self::delete(&results_view, |j| async {
								$job {
									job: &(j.into()),
									store,
								}
								.delete(self.cascade)
								.await
							})
							.await?;
						}

						if self.update
						{
							Self::update(&results_view, |j| async {
								$job {
									job: &(j.into()),
									store,
								}
								.update()
								.await
							})
							.await?;
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
									$job {
										job: &(j.into()),
										store,
									}
									.update()
									.await
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
									$job {
										job: &(j.into()),
										store,
									}
									.update()
									.await
								})
								.await?;
						}

						if export
						{
							let to_export =
								input::select(&results_view, "Select which Jobs you want to export")?;

							// WARN: this `let` seems redundant, but the "type needs to be known at this point"
							let export_result: DynResult<'_, _> =
								stream::iter(to_export.into_iter().map(Ok))
									.try_for_each_concurrent(None, |job| async move {
										let exported = markdown::job(&job)?;
										fs::write(
											format!(
												"{}--{}.md",
												job.client.name.replace(' ', "-"),
												job.id,
											),
											exported,
										)
										.await?;
										Ok(())
									})
									.await;
							export_result?;
						}
						else if !(close || self.delete || reopen || self.update)
						{
							results_view.iter().for_each(|j| println!("{}", j));
						}
					}};
				}

				match store.adapter
				{
					#[cfg(feature="postgres")]
					Adapters::Postgres => retrieve!(PostgresEmployee, PostgresJob, PostgresLocation, PostgresOrganization, PostgresPerson),

					_ => return adapter_not_enabled(),
				};
			},

			RetrieveCommand::Location { create_inner } =>
			{
				macro_rules! retrieve {
					($loc:ident) => {{
						let results_view = input::util::location::retrieve_view::<&str, $loc>(
							"Query the `Location` you are looking for",
							false,
							store,
						)
						.await?;

						if self.delete
						{
							let cascade = self.cascade;
							Self::delete(&results_view, |l| async {
								$loc {
									location: &(l.into()),
									store,
								}
								.delete(cascade)
								.await
							})
							.await?;
						}

						if self.update
						{
							Self::update(&results_view, |l| async {
								$loc {
									location: &(l.into()),
									store,
								}
								.update()
								.await
							})
							.await?;
						}

						if let Some(name) = create_inner.last()
						{
							let location = input::select_one(
								&results_view,
								format!("Select the outer Location of {}", name),
							)?;
							stream::iter(create_inner.into_iter().map(Ok).rev())
								.try_fold(location.into(), |loc: Location, name: String| async {
									$loc {
										location: &(loc.into()),
										store,
									}
									.create_inner(name)
									.await
								})
								.await?;
						}
						else if !(self.delete || self.update)
						{
							results_view.iter().for_each(|l| println!("{}", l));
						}
					}};
				}

				match store.adapter
				{
					#[cfg(feature="postgres")]
					Adapters::Postgres => retrieve!(PostgresLocation),

					_ => return adapter_not_enabled(),
				};
			},

			RetrieveCommand::Organization =>
			{
				macro_rules! retrieve {
					($loc:ident, $org:ident) => {{
						let results_view = input::util::organization::retrieve_view::<&str, $loc, $org>(
							"Query the `Organization` you are looking for",
							false,
							store,
						)
						.await?;

						if self.delete
						{
							Self::delete(&results_view, |o| async {
								$org {
									organization: &(o.into()),
									store,
								}
								.delete(self.cascade)
								.await
							})
							.await?;
						}

						if self.update
						{
							Self::update(&results_view, |o| async {
								$org {
									organization: &(o.into()),
									store,
								}
								.update()
								.await
							})
							.await?;
						}
						else if !self.delete
						{
							results_view.iter().for_each(|o| println!("{}", o));
						}
					}};
				}

				match store.adapter
				{
					#[cfg(feature="postgres")]
					Adapters::Postgres => retrieve!(PostgresLocation, PostgresOrganization),

					_ => return adapter_not_enabled(),
				};
			},

			RetrieveCommand::Person =>
			{
				macro_rules! retrieve {
					($per:ident) => {{
						let results_view = input::util::person::retrieve_view::<&str, $per>(
							"Query the `Person` you are looking for",
							false,
							store,
						)
						.await?;

						if self.delete
						{
							Self::delete(&results_view, |p| async {
								$per {
									person: &(p.into()),
									store,
								}
								.delete(self.cascade)
								.await
							})
							.await?;
						}

						if self.update
						{
							Self::update(&results_view, |p| async {
								$per {
									person: &(p.into()),
									store,
								}
								.update()
								.await
							})
							.await?;
						}
						else if !self.delete
						{
							results_view.iter().for_each(|p| println!("{}", p));
						}
					}};
				}

				match store.adapter
				{
					#[cfg(feature="postgres")]
					Adapters::Postgres => retrieve!(PostgresPerson),

					_ => return adapter_not_enabled(),
				};
			},
		};

		Ok(())
	}
}
