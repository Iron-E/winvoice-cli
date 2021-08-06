use core::fmt::Display;
use std::{error::Error, fs};

use clinvoice_adapter::{
	data::{Deletable, Error as DataError, LocationAdapter, Updatable},
	Adapters,
	Error as AdapterError,
};
#[cfg(feature = "bincode")]
use clinvoice_adapter_bincode::data::{
	BincodeEmployee,
	BincodeJob,
	BincodeLocation,
	BincodeOrganization,
	BincodePerson,
};
use clinvoice_data::{chrono::Utc, views::RestorableSerde, Location};
use clinvoice_export::Target;
use serde::{de::DeserializeOwned, Serialize};

use crate::{input, Config, DynResult, StructOpt};

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
			help = "Export retrieved entities to the specified format\nSupported: markdown",
			long,
			short
		)]
		export: Option<Target>,

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
	fn delete<'err, E, T>(
		entities: &[T],
		delete_entity: impl Fn(T) -> Result<(), E>,
	) -> DynResult<'err, ()>
	where
		E: Error + 'err,
		T: Clone + Display,
	{
		let selection = input::select(entities, "Select the entities you want to delete")?;
		selection
			.into_iter()
			.try_for_each(|entity| delete_entity(entity))
			.map_err(|e| e.into())
	}

	/// # Summary
	///
	/// Edit some `entities`, and then update them.
	///
	/// `update_entity` determines how the entities are updated.
	fn update<'err, E, T>(
		entities: &[T],
		update_entity: impl Fn(T) -> Result<(), E>,
	) -> DynResult<'err, ()>
	where
		E: Error + 'err,
		T: Clone + DeserializeOwned + Display + RestorableSerde + Serialize,
	{
		let selection = input::select(entities, "Select the entities you want to update")?;
		selection.into_iter().try_for_each(|entity| {
			let edited = match input::edit_and_restore(&entity, "Make any desired edits")
			{
				Ok(e) => e,
				Err(input::Error::NotEdited) => entity,
				Err(e) => return Err(e.into()),
			};

			update_entity(edited).map_err(|e| e.into())
		})
	}

	/// # Summary
	///
	/// Execute the constructed command.
	pub(super) async fn run<'err>(
		self,
		config: &Config<'_, '_>,
		store_name: String,
	) -> DynResult<'err, ()>
	{
		let store = config
			.get_store(&store_name)
			.expect("Storage name not known");

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
					($emp:ident, $loc:ident, $org:ident, $per:ident) => {{
						let results_view =
							input::util::employee::retrieve_views::<&str, $emp, $loc, $org, $per>(
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
								store,
							)?;

						if self.delete
						{
							Self::delete(&results_view, |e| {
								$emp {
									employee: &(e.into()),
									store,
								}
								.delete(self.cascade)
							})?;
						}

						if self.update
						{
							Self::update(&results_view, |e| {
								$emp {
									employee: &(e.into()),
									store,
								}
								.update()
							})?;
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
											DataError::NoData(format!("`{}`", stringify!(Employee)))
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

							new_config.update()?;
						}
						else if !(self.delete || self.update)
						{
							results_view.iter().for_each(|e| println!("{}", e));
						}
					}};
				}

				match store.adapter
				{
					#[cfg(feature = "bincode")]
					Adapters::Bincode => retrieve!(
						BincodeEmployee,
						BincodeLocation,
						BincodeOrganization,
						BincodePerson
					),

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
					($emp:ident, $job:ident, $loc:ident, $org:ident, $per:ident) => {{
						let results_view =
							input::util::job::retrieve_views::<&str, $emp, $job, $loc, $org, $per>(
								"Query the `Job` you are looking for",
								false,
								store,
							)?;

						if self.delete
						{
							Self::delete(&results_view, |j| {
								$job {
									job: &(j.into()),
									store,
								}
								.delete(self.cascade)
							})?;
						}

						if self.update
						{
							Self::update(&results_view, |j| {
								$job {
									job: &(j.into()),
									store,
								}
								.update()
							})?;
						}

						if close
						{
							let unclosed: Vec<_> = results_view
								.iter()
								.filter(|j| j.date_close.is_none())
								.cloned()
								.collect();
							let selected = input::select(&unclosed, "Select the Jobs you want to close")?;
							selected.into_iter().try_for_each(|mut j| {
								j.date_close = Some(Utc::now());
								$job {
									job: &(j.into()),
									store,
								}
								.update()
							})?;
						}

						if reopen
						{
							let closed: Vec<_> = results_view
								.iter()
								.filter(|j| j.date_close.is_some())
								.cloned()
								.collect();
							let selected = input::select(&closed, "Select the Jobs you want to reopen")?;
							selected.into_iter().try_for_each(|mut j| {
								j.date_close = None;
								$job {
									job: &(j.into()),
									store,
								}
								.update()
							})?;
						}

						if let Some(target) = export
						{
							input::select(&results_view, "Select which Jobs you want to export")?
								.into_iter()
								.try_for_each(|job| -> DynResult<()> {
									let exported = target.export_job(&job)?;
									fs::write(
										format!(
											"{}--{}{}",
											job.client.name.replace(' ', "-"),
											job.id,
											target.extension()
										),
										exported,
									)?;
									Ok(())
								})?;
						}
						else if !(close || self.delete || reopen || self.update)
						{
							results_view.iter().for_each(|j| println!("{}", j));
						}
					}};
				}

				match store.adapter
				{
					#[cfg(feature = "bincode")]
					Adapters::Bincode => retrieve!(
						BincodeEmployee,
						BincodeJob,
						BincodeLocation,
						BincodeOrganization,
						BincodePerson
					),

					_ => return adapter_not_enabled(),
				};
			},

			RetrieveCommand::Location { create_inner } =>
			{
				macro_rules! retrieve {
					($loc:ident) => {{
						let results_view = input::util::location::retrieve_views::<&str, $loc>(
							"Query the `Location` you are looking for",
							false,
							store,
						)?;

						if self.delete
						{
							let cascade = self.cascade;
							Self::delete(&results_view, |l| {
								$loc {
									location: &(l.into()),
									store,
								}
								.delete(cascade)
							})?;
						}

						if self.update
						{
							Self::update(&results_view, |l| {
								$loc {
									location: &(l.into()),
									store,
								}
								.update()
							})?;
						}

						if let Some(name) = create_inner.last()
						{
							let location = input::select_one(
								&results_view,
								format!("Select the outer Location of {}", name),
							)?;
							create_inner.into_iter().rev().try_fold(
								location.into(),
								|loc: Location,
								 name: String|
								 -> Result<Location, <$loc as LocationAdapter>::Error> {
									$loc {
										location: &(loc.into()),
										store,
									}
									.create_inner(name)
								},
							)?;
						}
						else if !(self.delete || self.update)
						{
							results_view.iter().for_each(|l| println!("{}", l));
						}
					}};
				}

				match store.adapter
				{
					#[cfg(feature = "bincode")]
					Adapters::Bincode => retrieve!(BincodeLocation),

					_ => return adapter_not_enabled(),
				};
			},

			RetrieveCommand::Organization =>
			{
				macro_rules! retrieve {
					($loc:ident, $org:ident) => {{
						let results_view = input::util::organization::retrieve_views::<&str, $loc, $org>(
							"Query the `Organization` you are looking for",
							false,
							store,
						)?;

						if self.delete
						{
							Self::delete(&results_view, |o| {
								$org {
									organization: &(o.into()),
									store,
								}
								.delete(self.cascade)
							})?;
						}

						if self.update
						{
							Self::update(&results_view, |o| {
								$org {
									organization: &(o.into()),
									store,
								}
								.update()
							})?;
						}
						else if !self.delete
						{
							results_view.iter().for_each(|o| println!("{}", o));
						}
					}};
				}

				match store.adapter
				{
					#[cfg(feature = "bincode")]
					Adapters::Bincode => retrieve!(BincodeLocation, BincodeOrganization),

					_ => return adapter_not_enabled(),
				};
			},

			RetrieveCommand::Person =>
			{
				macro_rules! retrieve {
					($per:ident) => {{
						let results_view = input::util::person::retrieve_views::<&str, $per>(
							"Query the `Person` you are looking for",
							false,
							store,
						)?;

						if self.delete
						{
							Self::delete(&results_view, |p| {
								$per {
									person: &(p.into()),
									store,
								}
								.delete(self.cascade)
							})?;
						}

						if self.update
						{
							Self::update(&results_view, |p| {
								$per {
									person: &(p.into()),
									store,
								}
								.update()
							})?;
						}
						else if !self.delete
						{
							results_view.iter().for_each(|p| println!("{}", p));
						}
					}};
				}

				match store.adapter
				{
					#[cfg(feature = "bincode")]
					Adapters::Bincode => retrieve!(BincodePerson),

					_ => return adapter_not_enabled(),
				};
			},
		};

		Ok(())
	}
}
