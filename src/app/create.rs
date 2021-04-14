use
{
	crate::{Config, DynResult, input, StructOpt},

	clinvoice_adapter::
	{
		Adapters, Error,
		data::{EmployeeAdapter, JobAdapter, LocationAdapter, OrganizationAdapter, PersonAdapter},
	},
	clinvoice_data::
	{
		chrono::{Datelike, DateTime, Local, Timelike, TimeZone, Utc},
		Decimal, EmployeeStatus, Money,
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
		#[structopt(help="The name of the location to create (e.g. 'Arizona')")]
		name: String,
	},

	#[structopt(about="Create a new organization record")]
	Organization
	{
		#[structopt(help="The name of the organization to create (e.g. 'FooCorp')")]
		name: String,
	},

	#[structopt(about="Create a new organization record")]
	Person
	{
		#[structopt(help="The name of the person to create (e.g. 'John')")]
		name: String,
	},
}

impl Create
{
	pub(super) fn run<'config>(self, config: &'config Config, store_name: String) -> DynResult<'config, ()>
	{
		let store = config.get_store(&store_name).expect("Storage name not known");

		match store.adapter
		{
			#[cfg(feature="bincode")]
			Adapters::Bincode => match self
			{
				Self::Employee {title} =>
				{
					let organization = input::util::organization::select_one::<BincodeLocation, BincodeOrganization, &str>(
						"Which organization does this employee work at?",
						store,
					)?.into();

					let people = input::util::person::retrieve_views::<BincodePerson>(store)?;
					let selected_person = input::select_one(&people, "Which person is working for the organization?")?;

					let contact_info = input::util::contact::creation_menu::<BincodeLocation>(store)?;
					let employee_status = input::select_one(
						&[EmployeeStatus::Employed, EmployeeStatus::NotEmployed, EmployeeStatus::Representative],
						"What is the status of the employee?",
					)?;

					BincodeEmployee::create(
						contact_info.into_iter().map(|(label, contact)| (label, contact.into())).collect(),
						organization,
						selected_person.into(),
						employee_status,
						&title,
						store,
					)?;

					Ok(())
				}

				Self::Job {currency, hourly_rate, year, month, day, hour, minute} =>
				{
					let client = input::util::organization::select_one::<BincodeLocation, BincodeOrganization, &str>(
						"Select the client for this job",
						store,
					)?;

					let objectives = input::edit_markdown("* List your objectives.\n* All markdown syntax works")?;

					BincodeJob::create(
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
						Money
						{
							amount: hourly_rate,
							currency: currency.unwrap_or_else(|| config.invoices.default_currency.into()),
						},
						&objectives,
						store,
					)?;

					Ok(())
				}

				Self::Location {name} => BincodeLocation::create(&name, store).and(Ok(())),

				Self::Organization {name} =>
				{
					let location_views = input::util::location::retrieve_views::<BincodeLocation>(store)?;
					let selected_view = input::select_one(&location_views, format!("Select a location for {}", name))?;

					BincodeOrganization::create(selected_view.into(), &name, store)?;

					Ok(())
				},

				Self::Person {name} => BincodePerson::create(&name, store).and(Ok(())),
			},

			_ => return Err(Error::FeatureNotFound(store.adapter).into()),
		}?;

		Ok(())
	}
}
