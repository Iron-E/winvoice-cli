use
{
	core::fmt::Display,
	std::{borrow::Cow::Borrowed, error::Error},

	super::QUERY_PROMPT,
	crate::{Config, DynResult, input, StructOpt},

	clinvoice_adapter::
	{
		Adapters, Error as AdapterError,
		data::{Deletable, EmployeeAdapter, Error as DataError, JobAdapter, LocationAdapter, OrganizationAdapter, PersonAdapter, Updatable},
	},
	clinvoice_data::
	{
		Location,
		views::{PersonView, RestorableSerde}
	},
	clinvoice_export::Target,
	clinvoice_query as query,

	serde::{de::DeserializeOwned, Serialize},
};

#[cfg(feature="bincode")]
use clinvoice_adapter_bincode::data::{BincodeEmployee, BincodeJob, BincodeLocation, BincodeOrganization, BincodePerson};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructOpt)]
#[structopt(about="Retrieve information that was recorded with CLInvoice")]
pub(super) struct Retrieve
{
	#[structopt(help="Select retrieved entities for deletion. See -c", long, short)]
	pub delete: bool,

	#[structopt(help="Cascade -d operations. Without this flag, entities referenced by other entities cannot be deleted", long, short)]
	pub cascade: bool,

	#[structopt(help="Select retrieved entities for data updating", long, short)]
	pub update: bool,

	#[structopt(subcommand)]
	pub command: RetrieveCommand,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructOpt)]
pub(super) enum RetrieveCommand
{
	#[structopt(about="Retrieve existing records about employees")]
	Employee
	{
		#[structopt(help="Retrieve the default employee as specified in your configuration", long, short)]
		default: bool,

		#[structopt(help="Set one of the employees as the default in your configuration", long, short)]
		set_default: bool,
	},

	#[structopt(about="Retrieve existing records about job")]
	Job
	{
		#[structopt(help="Export retrieved entities to the specified format.\nSupported: markdown", long, short)]
		export: Option<Target>,
	},

	#[structopt(about="Retrieve existing records about locations")]
	Location
	{
		#[structopt(help="Create a new location inside of some selected location. Argument is the same as `clinvoice create location`", long, short)]
		create_inner: Vec<String>,
	},

	#[structopt(about="Retrieve existing records about organizations")]
	Organization,

	#[structopt(about="Retrieve existing records about people")]
	Person,
}

impl Retrieve
{
	/// # Summary
	///
	/// Delete some `entities`
	///
	/// `delete_entity` determines how the entities are deleted.
	fn delete<'err, E, T>(entities: &[T], delete_entity: impl Fn(T) -> Result<(), E>) -> DynResult<'err, ()> where
		E : Error + 'err,
		T : Clone + Display,
	{
		let selection = input::select(entities, "Select the entities you want to delete")?;
		selection.into_iter().try_for_each(|entity| delete_entity(entity)).map_err(|e| e.into())
	}

	/// # Summary
	///
	/// Edit some `entities`, and then update them.
	///
	/// `update_entity` determines how the entities are updated.
	fn update<'err, E, T>(entities: &[T], update_entity: impl Fn(T) -> Result<(), E>) -> DynResult<'err, ()> where
		E : Error + 'err,
		T : Clone + DeserializeOwned + Display + RestorableSerde + Serialize,
	{
		let selection = input::select(entities, "Select the entities you want to update")?;
		selection.into_iter().try_for_each(|entity|
		{
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
	pub(super) fn run<'err>(self, config: &Config, store_name: String) -> DynResult<'err, ()>
	{
		let store = config.get_store(&store_name).expect("Storage name not known");

		let adapter_not_enabled = || -> DynResult<'err, ()>
		{
			Err(AdapterError::FeatureNotFound(store.adapter).into())
		};

		match self.command
		{
			RetrieveCommand::Employee {default, set_default} =>
			{
				macro_rules! retrieve
				{
					($emp: ident, $loc: ident, $org: ident, $per: ident) =>
					{{
						let query = match default
						{
							false => input::edit_default(String::from(QUERY_PROMPT) + "employees")?,
							_ => query::Employee
							{
								id: query::Match::EqualTo(Borrowed(&config.employees.default_id)),
								..Default::default()
							},
						};

						let results = $emp::retrieve(&query, &store)?;
						let results_view = results.into_iter().map(|e|
							$emp::into_view::<$loc, $org, $per>(e, &store)
						).filter_map(|result| match result
						{
							Ok(t) => match query.matches_view(&t)
							{
								Ok(b) if b => Some(Ok(t)),
								Err(e) => Some(Err(DataError::from(e).into())),
								_ => None,
							},
							Err(e) => Some(Err(e)),
						}).collect::<Result<Vec<_>, _>>()?;

						if self.delete
						{
							Self::delete(&results_view, |e| $emp {employee: &(e.into()), store}.delete(self.cascade))?;
						}

						if self.update
						{
							Self::update(&results_view, |e| $emp {employee: &(e.into()), store}.update())?;
						}

						if set_default
						{
							let mut new_config = config.clone();
							new_config.employees.default_id = match results_view.len() > 1
							{
								false => results_view.first().ok_or(DataError::NoData(stringify!(Employee)))?.id,
								_ => input::select_one(&results_view, "Which `Employee` should be the default?")?.id,
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
					#[cfg(feature="bincode")]
					Adapters::Bincode => retrieve!(BincodeEmployee, BincodeLocation, BincodeOrganization, BincodePerson),

					_ => return adapter_not_enabled(),
				};
			},

			RetrieveCommand::Job {export} =>
			{
				macro_rules! retrieve
				{
					($emp: ident, $job: ident, $loc: ident, $org: ident, $per: ident) =>
					{{
						let query: query::Job = input::edit_default(String::from(QUERY_PROMPT) + "jobs")?;

						let results = $job::retrieve(&query, &store)?;
						let results_view = results.into_iter().map(|j|
							$job::into_view::<$emp, $loc, $org, $per>(j, &store)
						).filter_map(|result| match result
						{
							Ok(t) => match query.matches_view(&t)
							{
								Ok(b) if b => Some(Ok(t)),
								Err(e) => Some(Err(DataError::from(e).into())),
								_ => None,
							},
							Err(e) => Some(Err(e)),
						}).collect::<Result<Vec<_>, _>>()?;

						if self.delete
						{
							Self::delete(&results_view, |j| $job {job: &(j.into()), store}.delete(self.cascade))?;
						}

						if self.update
						{
							Self::update(&results_view, |j| $job {job: &(j.into()), store}.update())?;
						}

						if let Some(target) = export
						{
							input::select(&results_view, "Select which Jobs you want to export")?.into_iter().for_each(|job|
								println!("{}", target.export_job(&job))
							);
						}
						else if !(self.delete || self.update)
						{
							results_view.iter().for_each(|j| println!("{}", j));
						}
					}};
				}

				match store.adapter
				{
					#[cfg(feature="bincode")]
					Adapters::Bincode => retrieve!(BincodeEmployee, BincodeJob, BincodeLocation, BincodeOrganization, BincodePerson),

					_ => return adapter_not_enabled(),
				};
			},

			RetrieveCommand::Location {ref create_inner} =>
			{
				macro_rules! retrieve
				{
					($loc: ident) =>
					{{
						let query: query::Location = input::edit_default(String::from(QUERY_PROMPT) + "locations")?;

						let results = $loc::retrieve(&query, &store)?;
						let results_view = results.into_iter().map(|l|
							$loc::into_view(l, &store)
						).filter_map(|result| match result
						{
							Ok(t) => match query.matches_view(&t)
							{
								Ok(b) if b => Some(Ok(t)),
								Err(e) => Some(Err(DataError::from(e).into())),
								_ => None,
							},
							Err(e) => Some(Err(e)),
						}).collect::<Result<Vec<_>, _>>()?;

						if self.delete
						{
							Self::delete(&results_view, |l| $loc {location: &(l.into()), store}.delete(self.cascade))?;
						}

						if self.update
						{
							Self::update(&results_view, |l| $loc {location: &(l.into()), store}.update())?;
						}

						if let Some(name) = create_inner.first()
						{
							let location = input::select_one(&results_view, format!("Select the outer Location of {}", name))?;
							create_inner.iter().try_fold(location.into(),
								|loc: Location, name: &String| -> Result<Location, <$loc as LocationAdapter>::Error>
								{
									Ok($loc {location: &(loc.into()), store}.create_inner(name)?)
								}
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
					#[cfg(feature="bincode")]
					Adapters::Bincode => retrieve!(BincodeLocation),

					_ => return adapter_not_enabled(),
				};
			},

			RetrieveCommand::Organization =>
			{
				macro_rules! retrieve
				{
					($loc: ident, $org: ident) =>
					{{
						let query: query::Organization = input::edit_default(String::from(QUERY_PROMPT) + "organizations")?;

						let results = $org::retrieve(&query, &store)?;
						let results_view = results.into_iter().map(|o|
							$org::into_view::<$loc>(o, &store)
						).filter_map(|result| match result
						{
							Ok(t) => match query.matches_view(&t)
							{
								Ok(b) if b => Some(Ok(t)),
								Err(e) => Some(Err(DataError::from(e).into())),
								_ => None,
							},
							Err(e) => Some(Err(e)),
						}).collect::<Result<Vec<_>, _>>()?;

						if self.delete
						{
							Self::delete(&results_view, |o| $org {organization: &(o.into()), store}.delete(self.cascade))?;
						}

						if self.update
						{
							Self::update(&results_view, |o| $org {organization: &(o.into()), store}.update())?;
						}
						else if !self.delete
						{
							results_view.iter().for_each(|o| println!("{}", o));
						}
					}};
				}

				match store.adapter
				{
					#[cfg(feature="bincode")]
					Adapters::Bincode => retrieve!(BincodeLocation, BincodeOrganization),

					_ => return adapter_not_enabled(),
				};
			},

			RetrieveCommand::Person =>
			{
				macro_rules! retrieve
				{
					($per: ident) =>
					{{
						let query: query::Person = input::edit_default(String::from(QUERY_PROMPT) + "persons")?;

						let results = $per::retrieve(&query, &store)?;
						let results_view = results.into_iter().map(PersonView::from).filter_map(|view|
							match query.matches_view(&view)
							{
								Ok(b) if b => Some(Ok(view)),
								Err(e) => Some(Err(e)),
								_ => None,
							}
						).collect::<Result<Vec<_>, _>>()?;

						if self.delete
						{
							Self::delete(&results_view, |p| $per {person: &(p.into()), store}.delete(self.cascade))?;
						}

						if self.update
						{
							Self::update(&results_view, |p| $per {person: &(p.into()), store}.update())?;
						}
						else if !self.delete
						{
							results_view.iter().for_each(|p| println!("{}", p));
						}
					}};
				}

				match store.adapter
				{
					#[cfg(feature="bincode")]
					Adapters::Bincode => retrieve!(BincodePerson),

					_ => return adapter_not_enabled(),
				};
			},
		};

		Ok(())
	}
}
