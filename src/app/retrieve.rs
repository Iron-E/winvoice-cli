use
{
	crate::{Config, DynResult, io::input, StructOpt},

	clinvoice_adapter::
	{
		Adapters, Error,
		data::{Deletable, EmployeeAdapter, JobAdapter, LocationAdapter, OrganizationAdapter, PersonAdapter, query},
	},
	clinvoice_data::views::PersonView,
	clinvoice_export::Target,
};

#[cfg(feature="bincode")]
use clinvoice_adapter_bincode::data::{BincodeEmployee, BincodeJob, BincodeLocation, BincodeOrganization, BincodePerson, Result as BincodeResult};

/// # Summary
///
/// The prompt for when editing a [query](clinvoice_adapter::data::query).
const QUERY_PROMPT: &str = "See the documentation of this query at https://github.com/Iron-E/clinvoice/wiki/Query-Syntax#";

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
		select_default: bool,
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
		#[structopt(help="Create a new location inside of some selected location", long, short)]
		create_inner: bool,
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
	/// Execute the constructed command.
	pub(super) fn run<'config>(self, config: &'config Config, store_name: String) -> DynResult<'config, ()>
	{
		let store = config.get_store(&store_name).expect("Storage name not known");

		match store.adapter
		{
			#[cfg(feature="bincode")]
			Adapters::Bincode => match self.command
			{
				RetrieveCommand::Employee {select_default} =>
				{
					let query: query::Employee = input::edit_default(String::from(QUERY_PROMPT) + "employees")?;

					let results = BincodeEmployee::retrieve(query, &store)?;
					results.into_iter().try_for_each(|employee| -> BincodeResult<()>
					{
						let view = BincodeEmployee::to_view::<BincodeLocation, BincodeOrganization, BincodePerson>(
							employee,
							&store,
						)?;

						println!("{}", view);

						Ok(())
					})?;
				},

				RetrieveCommand::Job {export} =>
				{
					let query: query::Job = input::edit_default(String::from(QUERY_PROMPT) + "jobs")?;

					let results = BincodeJob::retrieve(query, &store)?;
					results.into_iter().try_for_each(|job| -> BincodeResult<()>
					{
						let view = BincodeJob::to_view::<BincodeEmployee, BincodeLocation, BincodeOrganization, BincodePerson>(
							job,
							&store,
						)?;

						println!("{}", view);

						Ok(())
					})?;
				},

				RetrieveCommand::Location {create_inner} =>
				{
					let query: query::Location = input::edit_default(String::from(QUERY_PROMPT) + "locations")?;

					let results = BincodeLocation::retrieve(query, &store)?;
					results.into_iter().try_for_each(|job| -> BincodeResult<()>
					{
						let view = BincodeLocation::to_view(
							job,
							&store,
						)?;

						println!("{}", view);

						Ok(())
					})?;
				},

				RetrieveCommand::Organization =>
				{
					let query: query::Organization = input::edit_default(String::from(QUERY_PROMPT) + "organizations")?;

					let results = BincodeOrganization::retrieve(query, &store)?;
					results.into_iter().try_for_each(|job| -> BincodeResult<()>
					{
						let view = BincodeOrganization::to_view::<BincodeLocation>(
							job,
							&store,
						)?;

						println!("{}", view);

						Ok(())
					})?;
				},

				RetrieveCommand::Person =>
				{
					let query: query::Person = input::edit_default(String::from(QUERY_PROMPT) + "persons")?;

					let results = BincodePerson::retrieve(query, &store)?;
					results.iter().for_each(|person|
					{
						let view: PersonView = person.into();
						println!("{}", view);
					});
				}
			},

			_ => return Err(Error::FeatureNotFound(store.adapter).into()),
		};

		Ok(())
	}
}
