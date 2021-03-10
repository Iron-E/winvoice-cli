use
{
	crate::{Config, DynResult, io::input::util as input_util, StructOpt},
	clinvoice_adapter::
	{
		Adapters, Error,
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
		title: String,
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
		#[structopt(about="The name of the organization to create.")]
		name: String,
	},

	#[structopt(about="Create a new organization record")]
	Person
	{
		#[structopt(about="The name of the person to create.")]
		name: String,
	},
}

impl Create
{
	pub(super) fn run<'store>(self, config: &'store Config, store_name: String) -> DynResult<'store, ()>
	{
		let store = config.get_store(&store_name).expect("Storage name not known.");

		match store.adapter
		{
			#[cfg(feature="bincode")]
			Adapters::Bincode => match self
			{
				Self::Employee {title} => BincodeEmployee::create(
					input_util::contact::select::<BincodeLocation>(store)?,
					input_util::organization::select_one::<BincodeOrganization, &str>("Which organization does this employee work at?", store)?.into(),
					input_util::person::select_one::<BincodePerson, &str>("Which person is working for the organization?", store)?.into(),
					input_util::employee_status::select_one("What is the status of the employee?")?,
					&title,
					store,
				).and(Ok(())),

				Self::Job {} => todo!() /*BincodeJob::create(store).and(Ok(()))*/,

				Self::Location {name} => BincodeLocation::create(&name, store).and(Ok(())),

				Self::Organization {name} => BincodeOrganization::create(
					input_util::location::select_one::<BincodeLocation, String>(format!("Select a Location for {}", name), store)?.into(),
					&name,
					store
				).and(Ok(())),

				Self::Person {name} => BincodePerson::create(
					input_util::contact::select::<BincodeLocation>(store)?,
					&name,
					store,
				).and(Ok(())),
			},

			_ => return Err(Error::FeatureNotFound {adapter: store.adapter}.into()),
		}?;

		Ok(())
	}
}
