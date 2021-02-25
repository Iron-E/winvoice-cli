use
{
	crate::{Config, io, StructOpt},
	clinvoice_adapter::
	{
		Adapters, DynamicResult, Error,
		data::{EmployeeAdapter, JobAdapter, LocationAdapter, OrganizationAdapter, PersonAdapter},
	},
};

#[cfg(feature="bincode")]
use clinvoice_adapter_bincode::data::{BincodeEmployee, BincodeJob, BincodeLocation, BincodeOrganization, BincodePerson};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructOpt)]
#[structopt(about="Record information information with CLInvoice")]
pub(super) enum Create
{
	#[structopt(about="Create a new employee record")]
	Employee
	{
	},

	#[structopt(about="Create a new job record")]
	Job
	{
	},

	#[structopt(about="Create a new location record")]
	Location
	{
		#[structopt(about="The name of the location to create.")]
		name: String,
	},

	#[structopt(about="Create a new organization record")]
	Organization
	{
	},

	#[structopt(about="Create a new organization record")]
	Person
	{
		#[structopt(about="The name of the person to create.")]
		name: String,
	},
}

impl<'pass, 'path, 'user> Create
{
	pub(super) fn run(self, config: Config, store_name: &str) -> DynamicResult<()>
	{
		let store = config.get_store(store_name).expect("Storage name not known.");

		match store.adapter
		{
			#[cfg(feature="bincode")]
			Adapters::Bincode => match self
			{
				Self::Employee {} => todo!() /*BincodeEmployee::create(*store).and(Ok(()))*/,

				Self::Job {} => todo!() /*BincodeJob::create(*store).and(Ok(()))*/,

				Self::Location {name} => BincodeLocation::create(&name, *store).and(Ok(())),

				Self::Organization {} => todo!() /*BincodeOrganization::create(*store).and(Ok(()))*/,

				Self::Person {name} => BincodePerson::create(
					io::input::util::contact_info::<BincodeLocation>(*store)?,
					&name,
					*store,
				).and(Ok(())),
			},

			_ => return Err(Error::FeatureNotFound {adapter: store.adapter}.into()),
		}?;

		return Ok(());
	}
}
