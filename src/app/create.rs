use
{
	crate::{Config, DynResult, io::input, StructOpt},
	clinvoice_adapter::
	{
		Adapters, Error,
		data::{EmployeeAdapter, JobAdapter, LocationAdapter, OrganizationAdapter, PersonAdapter},
	},
	clinvoice_data::
	{
		chrono::{DateTime, Local, Utc},
		Decimal, Money,
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
	Job,

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
				Self::Employee {title} =>
				{
					let organization = input::util::organization::select_one::<BincodeOrganization, &str>(
						"Which organization does this employee work at?",
						store,
					)?.into();

					let person = input::util::person::select_one::<BincodePerson, &str>(
						"Which person is working for the organization?",
						store,
					)?.into();

					BincodeEmployee::create(
						input::util::contact::edit_select::<BincodeLocation>(store)?,
						organization,
						person,
						input::util::employee_status::select_one("What is the status of the employee?")?,
						&title,
						store,
					).and(Ok(()))
				}

				Self::Job => BincodeJob::create(
					input::util::organization::select_one::<BincodeOrganization, &str>("", store)?.into(),
					DateTime::<Utc>::from(input::edit(
							Some(""),
							Local::now()
					)?),
					input::edit(Some(""), Money::new(Decimal::new(2000, 2), "USD"))?,
					&input::edit_markdown("* List your objectives.\n* All markdown syntax works.")?,
					store,
				).and(Ok(())),

				Self::Location {name} => BincodeLocation::create(&name, store).and(Ok(())),

				Self::Organization {name} => BincodeOrganization::create(
					input::util::location::select_one::<BincodeLocation, String>(format!("Select a Location for {}", name), store)?.into(),
					&name,
					store
				).and(Ok(())),

				Self::Person {name} => BincodePerson::create(
					input::util::contact::edit_select::<BincodeLocation>(store)?,
					&name,
					store,
				).and(Ok(())),
			},

			_ => return Err(Error::FeatureNotFound {adapter: store.adapter}.into()),
		}?;

		Ok(())
	}
}
