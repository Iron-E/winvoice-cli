use
{
	core::fmt::Display,
	std::{borrow::Cow, error::Error, result},

	crate::{Config, DynResult, io::input, StructOpt},

	clinvoice_adapter::
	{
		Adapters, Error as AdapterError,
		data::{Deletable, EmployeeAdapter, JobAdapter, LocationAdapter, Match, OrganizationAdapter, PersonAdapter, query, Updatable},
	},
	clinvoice_data::views::{PersonView, RestorableSerde},
	clinvoice_export::Target,

	serde::{de::DeserializeOwned, Serialize},
};

#[cfg(feature="bincode")]
use clinvoice_adapter_bincode::data::{BincodeEmployee, BincodeJob, BincodeLocation, BincodeOrganization, BincodePerson, Result as BincodeResult};

/// # Summary
///
/// The prompt for when editing a [query](clinvoice_adapter::data::query).
const QUERY_PROMPT: &str = "See the documentation of this query at https://github.com/Iron-E/clinvoice/wiki/Query-Syntax#";

type Result<E> = result::Result<(), E>;

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
		#[structopt(help="Select one of the employees as the default in your configuration", long, short)]
		default: bool,
	},

	#[structopt(about="Retrieve existing records about job")]
	Job
	{
		#[structopt(default_value="markdown", help="Export retrieved entities to the specified format.\nSupported: markdown", long, short)]
		export: Target,
	},

	#[structopt(about="Retrieve existing records about locations")]
	Location
	{
		#[structopt(help="Create a new location inside of some selected location. Argument is the name of the new location", long, short)]
		create_inner: Option<String>,
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
	fn delete<'err, E, T>(entities: &[T], delete_entity: impl Fn(T) -> Result<E>) -> DynResult<'err, ()> where
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
	fn update<'err, E, T>(entities: &[T], update_entity: impl Fn(T) -> Result<E>) -> DynResult<'err, ()> where
		E : Error + 'err,
		T : Clone + DeserializeOwned + Display + RestorableSerde + Serialize,
	{
		let selection = input::select(entities, "Select the entities you want to update")?;
		selection.into_iter().try_for_each(|entity|
		{
			let edited = match input::edit_and_restore("Edit ", &entity)
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
	pub(super) fn run<'config>(self, config: &'config Config, store_name: String) -> DynResult<'config, ()>
	{
		let store = config.get_store(&store_name).expect("Storage name not known");

		match store.adapter
		{
			#[cfg(feature="bincode")]
			Adapters::Bincode => match self.command
			{
				RetrieveCommand::Employee {default} =>
				{
					let query = if default
					{
						query::Employee
						{
							id: Match::EqualTo(Cow::Borrowed(&config.employees.default_id)),
							..Default::default()
						}
					}
					else
					{
						input::edit_default(String::from(QUERY_PROMPT) + "employees")?
					};

					let results = BincodeEmployee::retrieve(query, &store)?;
					let results_len = results.len();
					let results_view = results.into_iter().try_fold(
						Vec::with_capacity(results_len),
						|mut v, e| -> BincodeResult<_>
						{
							v.push(BincodeEmployee::into_view::<BincodeLocation, BincodeOrganization, BincodePerson>(e, &store)?);
							Ok(v)
						}
					)?;

					if self.delete
					{
						Self::delete(&results_view, |e| BincodeEmployee {employee: &(e.into()), store}.delete(self.cascade))?;
					}

					if self.update
					{
						Self::update(&results_view, |e| BincodeEmployee {employee: &(e.into()), store}.update())?;
					}
					else if !self.delete
					{
						results_view.iter().for_each(|e| println!("{}", e));
					}
				},

				RetrieveCommand::Job {export} =>
				{
					let query: query::Job = input::edit_default(String::from(QUERY_PROMPT) + "jobs")?;

					let results = BincodeJob::retrieve(query, &store)?;
					let results_len = results.len();
					let results_view = results.into_iter().try_fold(
						Vec::with_capacity(results_len),
						|mut v, j| -> BincodeResult<_>
						{
							v.push(BincodeJob::into_view::<BincodeEmployee, BincodeLocation, BincodeOrganization, BincodePerson>(j, &store)?);
							Ok(v)
						}
					)?;

					if self.delete
					{
						Self::delete(&results_view, |j| BincodeJob {job: &(j.into()), store}.delete(self.cascade))?;
					}

					if self.update
					{
						Self::update(&results_view, |j| BincodeJob {job: &(j.into()), store}.update())?;
					}
					else if !self.delete
					{
						results_view.iter().for_each(|j| println!("{}", j));
					}
				},

				RetrieveCommand::Location {ref create_inner} =>
				{
					let query: query::Location = input::edit_default(String::from(QUERY_PROMPT) + "locations")?;

					let results = BincodeLocation::retrieve(query, &store)?;
					let results_len = results.len();
					let results_view = results.into_iter().try_fold(
						Vec::with_capacity(results_len),
						|mut v, l| -> BincodeResult<_>
						{
							v.push(BincodeLocation::into_view(l, &store)?);
							Ok(v)
						}
					)?;

					if self.delete
					{
						Self::delete(&results_view, |l| BincodeLocation {location: &(l.into()), store}.delete(self.cascade))?;
					}

					if self.update
					{
						Self::update(&results_view, |l| BincodeLocation {location: &(l.into()), store}.update())?;
					}

					if let Some(name) = create_inner
					{
						let location = input::select_one(&results_view, format!("Select the outer Location of {}", name))?;
						BincodeLocation {location: &(location.into()), store}.create_inner(name.as_str())?;
					}
					else if !(self.delete || self.update)
					{
						results_view.iter().for_each(|l| println!("{}", l));
					}
				},

				RetrieveCommand::Organization =>
				{
					let query: query::Organization = input::edit_default(String::from(QUERY_PROMPT) + "organizations")?;

					let results = BincodeOrganization::retrieve(query, &store)?;
					let results_len = results.len();
					let results_view = results.into_iter().try_fold(
						Vec::with_capacity(results_len),
						|mut v, o| -> BincodeResult<_>
						{
							v.push(BincodeOrganization::into_view::<BincodeLocation>(o, &store)?);
							Ok(v)
						}
					)?;

					if self.delete
					{
						Self::delete(&results_view, |o| BincodeOrganization {organization: &(o.into()), store}.delete(self.cascade))?;
					}

					if self.update
					{
						Self::update(&results_view, |o| BincodeOrganization {organization: &(o.into()), store}.update())?;
					}
					else if !self.delete
					{
						results_view.iter().for_each(|o| println!("{}", o));
					}
				},

				RetrieveCommand::Person =>
				{
					let query: query::Person = input::edit_default(String::from(QUERY_PROMPT) + "persons")?;

					let results = BincodePerson::retrieve(query, &store)?;
					let results_view = results.into_iter().map(PersonView::from).collect::<Vec<_>>();

					if self.delete
					{
						Self::delete(&results_view, |p| BincodePerson {person: &(p.into()), store}.delete(self.cascade))?;
					}

					if self.update
					{
						Self::update(&results_view, |p| BincodePerson {person: &(p.into()), store}.update())?;
					}
					else if !self.delete
					{
						results_view.iter().for_each(|p| println!("{}", p));
					}
				}
			},

			_ => return Err(AdapterError::FeatureNotFound(store.adapter).into()),
		};

		Ok(())
	}
}
