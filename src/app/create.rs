use
{
	crate::{Config, DynResult, input, StructOpt},

	clinvoice_adapter::
	{
		Adapters, Error,
		data::{EmployeeAdapter, JobAdapter, LocationAdapter, OrganizationAdapter, PersonAdapter},
		Store,
	},
	clinvoice_data::
	{
		chrono::{Datelike, DateTime, Local, Timelike, TimeZone, Utc},
		Decimal, EmployeeStatus, Location, Money,
	},
};

#[cfg(feature="bincode")]
use clinvoice_adapter_bincode::data::{BincodeEmployee, BincodeJob, BincodeLocation, BincodeOrganization, BincodePerson, Error as BincodeError};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructOpt)]
#[structopt(about="Record information information with CLInvoice")]
pub(super) enum Create
{
	#[structopt(about="Create a new employee record")]
	Employee
	{
		#[structopt(help="The job title of the employee (e.g. 'Hal'")]
		title: String,
	},

	#[structopt(about="Create a new job record")]
	Job
	{
		#[structopt(help="The currency which the hourly rate is stated in (e.g. 'USD')", long, short)]
		currency: Option<String>,

		#[structopt(help="The amount of money charged per hour for this job (e.g. 12.00)")]
		hourly_rate: Decimal,

		#[structopt(help="The year that the job was created (e.g. 2021). Defaults to current year", requires("month"))]
		year: Option<i32>,

		#[structopt(help="The month that the job was created (e.g. 4 for 'April'). Defaults to current month", requires("day"))]
		month: Option<u32>,

		#[structopt(help="The day that the job was created (e.g. 21). Defaults to current day")]
		day: Option<u32>,

		#[structopt(help="The hour that the job was created (e.g. 13 for 1pm). Defaults to current hour", requires("minute"))]
		hour: Option<u32>,

		#[structopt(help="The minute that the job was created (e.g. 45). Defaults to current minute")]
		minute: Option<u32>,
	},

	#[structopt(about="Create a new location record")]
	Location
	{
		#[structopt(help="The name of the location to create (e.g. 'Arizona').\nProvide multiple names to create a hierarchy (e.g. 'Arizona' 'United States')", required=true)]
		names: Vec<String>,
	},

	#[structopt(about="Create a new organization record")]
	Organization
	{
		#[structopt(help="The name of the organization to create (e.g. 'FooCorp')")]
		name: String,
	},

	#[structopt(about="Create a new person record")]
	Person
	{
		#[structopt(help="The name of the person to create (e.g. 'John')")]
		name: String,
	},
}

impl Create
{
	fn create_employee<'err, E, L, O, P>(title: String, store: &Store) -> DynResult<'err, ()> where
		E : EmployeeAdapter,
		L : LocationAdapter,
		O : OrganizationAdapter,
		P : PersonAdapter,

		<E as EmployeeAdapter>::Error : 'err,
		<L as LocationAdapter>::Error : 'err,
		<O as OrganizationAdapter>::Error : 'err,
		<P as PersonAdapter>::Error : 'err,
	{
		let organization_views = input::util::organization::retrieve_views::<L, O>(store)?;
		let organization = input::select_one(&organization_views, "Which organization does this employee work at?")?;

		let person_views = input::util::person::retrieve_views::<P>(store)?;
		let person = input::select_one(&person_views, "Which person is working for the organization?")?;

		let contact_info = input::util::contact::menu::<L>(store)?;
		let employee_status = input::select_one(
			&[EmployeeStatus::Employed, EmployeeStatus::NotEmployed, EmployeeStatus::Representative],
			"What is the status of the employee?",
		)?;

		E::create(
			contact_info.into_iter().map(|(label, contact)| (label, contact.into())).collect(),
			organization.into(),
			person.into(),
			employee_status,
			title,
			store,
		)?;

		Ok(())
	}

	fn create_job<'err, J, L, O>(
		currency: String,
		hourly_rate: Decimal,
		year: Option<i32>,
		month: Option<u32>,
		day: Option<u32>,
		hour: Option<u32>,
		minute: Option<u32>,
		store: &Store,
	) -> DynResult<'err, ()> where
		J : JobAdapter,
		L : LocationAdapter,
		O : OrganizationAdapter,

		<J as JobAdapter>::Error : 'err,
		<L as LocationAdapter>::Error : 'err,
		<O as OrganizationAdapter>::Error : 'err,
	{
		let organization_views = input::util::organization::retrieve_views::<L, O>(store)?;
		let client = input::select_one(&organization_views, "Select the client for this job")?;

		let objectives = input::edit_markdown("* List your objectives.\n* All markdown syntax works")?;

		J::create(
			client.into(),
			DateTime::<Utc>::from(
			{
				let now = Local::now();

				// This should be valid because of the `requires` on `Job`. Either all are present or none.
				let date = Local.ymd(
					year.unwrap_or_else(|| now.year()),
					month.unwrap_or_else(|| now.month()),
					day.unwrap_or_else(|| now.day()),
				);

				match year
				{
					Some(_) => date.and_hms(0, 0, 0),
					None => date.and_hms(
						hour.unwrap_or_else(|| now.hour()),
						minute.unwrap_or_else(|| now.minute()),
						0,
					)
				}
			}),
			Money {amount: hourly_rate, currency},
			objectives,
			store,
		)?;

		Ok(())
	}

	fn create_location<L>(
		create_inner: fn(&Location, String, &Store) -> Result<Location, <L as LocationAdapter>::Error>,
		names: Vec<String>,
		store: &Store,
	) -> Result<(), <L as LocationAdapter>::Error>
	where
		L : LocationAdapter,
	{
		if let Some(name) = names.last()
		{
			let outer = L::create(name.clone(), store)?;
			names.into_iter().rev().skip(1).try_fold(outer, |outer, name| -> Result<Location, <L as LocationAdapter>::Error>
			{
				create_inner(&outer, name, store)
			})?;
		}

		Ok(())
	}

	fn create_organization<'err, L, O>(name: String, store: &Store) -> DynResult<'err, ()> where
		L : LocationAdapter,
		O : OrganizationAdapter,

		<L as LocationAdapter>::Error : 'err,
		<O as OrganizationAdapter>::Error : 'err,
	{
		let location_views = input::util::location::retrieve_views::<L>(store)?;
		let selected_view = input::select_one(&location_views, format!("Select a location for {}", name))?;

		O::create(selected_view.into(), name, store)?;

		Ok(())
	}

	pub(super) fn run<'config>(self, config: &'config Config, store_name: String) -> DynResult<'config, ()>
	{
		let store = config.get_store(&store_name).expect("Storage name not known");

		match store.adapter
		{
			#[cfg(feature="bincode")]
			Adapters::Bincode => match self
			{
				Self::Employee {title} =>
					Self::create_employee::<BincodeEmployee, BincodeLocation, BincodeOrganization, BincodePerson>(title, store),

				Self::Job {currency, hourly_rate, year, month, day, hour, minute} =>
					Self::create_job::<BincodeJob, BincodeLocation, BincodeOrganization>(
						currency.unwrap_or_else(|| config.invoices.default_currency.into()),
						hourly_rate, year, month, day, hour, minute,
						store,
					),

				Self::Location {names} =>
				{
					fn create_inner(location: &Location, name: String, store: &Store) -> Result<Location, BincodeError>
					{
						BincodeLocation {location, store}.create_inner(name)
					}

					Self::create_location::<BincodeLocation>(create_inner, names, store).map_err(|e| e.into())
				},

				Self::Organization {name} =>
					Self::create_organization::<BincodeLocation, BincodeOrganization>(name, store),

				Self::Person {name} =>
					BincodePerson::create(name, store).and(Ok(())).map_err(|e| e.into()),
			},

			_ => return Err(Error::FeatureNotFound(store.adapter).into()),
		}?;

		Ok(())
	}
}
