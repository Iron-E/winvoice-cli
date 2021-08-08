use clinvoice_adapter::{
	data::{EmployeeAdapter, JobAdapter, LocationAdapter, OrganizationAdapter, PersonAdapter},
	Adapters,
	Error,
	Store,
};
#[cfg(feature = "bincode")]
use clinvoice_adapter_bincode::data::{
	BincodeEmployee,
	BincodeJob,
	BincodeLocation,
	BincodeOrganization,
	BincodePerson,
};
use clinvoice_data::{
	chrono::{Datelike, Local, TimeZone, Timelike},
	finance::{Currency, Decimal, Money},
	EmployeeStatus,
	Location,
};
use futures::{
	stream::{self, TryStreamExt},
	Future,
	TryFutureExt,
};

use crate::{input, Config, DynResult, StructOpt};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructOpt)]
#[structopt(about = "Record information information with CLInvoice")]
pub(super) enum Create
{
	#[structopt(about = "Create a new employee record")]
	Employee
	{
		#[structopt(help = "The job title of the employee (e.g. 'Hal'")]
		title: String,
	},

	#[structopt(about = "Create a new job record")]
	Job
	{
		#[structopt(
			help = "The currency which the hourly rate is stated in (e.g. 'USD')\nDefaults to the \
			        value set in your config",
			long,
			short
		)]
		currency: Option<Currency>,

		#[structopt(help = "The amount of money charged per hour for this job (e.g. 12.00)")]
		hourly_rate: Decimal,

		#[structopt(
			help = "The year that the job was created (e.g. 2021)\nDefaults to current year",
			requires("month")
		)]
		year: Option<i32>,

		#[structopt(
			help = "The month that the job was created (e.g. 4 for 'April')\nDefaults to current \
			        month",
			requires("day")
		)]
		month: Option<u32>,

		#[structopt(help = "The day that the job was created (e.g. 21)\nDefaults to current day")]
		day: Option<u32>,

		#[structopt(
			help = "The hour that the job was created (e.g. 13 for 1pm)\nDefaults to current hour",
			requires("minute")
		)]
		hour: Option<u32>,

		#[structopt(
			help = "The minute that the job was created (e.g. 45)\nDefaults to current minute"
		)]
		minute: Option<u32>,
	},

	#[structopt(about = "Create a new location record")]
	Location
	{
		#[structopt(
			help = "The name of the location to create (e.g. 'Arizona')\nProvide multiple names to \
			        create a hierarchy (e.g. 'Arizona' 'United States')",
			required = true
		)]
		names: Vec<String>,
	},

	#[structopt(about = "Create a new organization record")]
	Organization
	{
		#[structopt(help = "The name of the organization to create (e.g. 'FooCorp')")]
		name: String,
	},

	#[structopt(about = "Create a new person record")]
	Person
	{
		#[structopt(help = "The name of the person to create (e.g. 'John')")]
		name: String,
	},
}

impl Create
{
	async fn create_employee<'err, E, L, O, P>(title: String, store: &Store) -> DynResult<'err, ()>
	where
		E: EmployeeAdapter,
		L: LocationAdapter + Send,
		O: OrganizationAdapter + Send,
		P: PersonAdapter,

		<E as EmployeeAdapter>::Error: 'err,
		<L as LocationAdapter>::Error: 'err,
		<O as OrganizationAdapter>::Error: 'err,
		<P as PersonAdapter>::Error: 'err,
	{
		let organization_views = input::util::organization::retrieve_views::<&str, L, O>(
			"Query the `Organization` where this `Employee` works",
			false,
			store,
		)
		.await?;

		let organization = input::select_one(
			&organization_views,
			"Which organization does this employee work at?",
		)?;

		let person_views = input::util::person::retrieve_views::<&str, P>(
			"Query the `Person` who this `Employee` is",
			true,
			store,
		)
		.await?;

		let person = input::select_one(&person_views, "Which `Person` is this `Employee`?")?;

		let contact_info = input::util::contact::menu::<L>(store).await?;
		let employee_status = input::select_one(
			&[
				EmployeeStatus::Employed,
				EmployeeStatus::NotEmployed,
				EmployeeStatus::Representative,
			],
			"What is the status of the employee?",
		)?;

		E::create(
			contact_info
				.into_iter()
				.map(|(label, contact)| (label, contact.into()))
				.collect(),
			organization.into(),
			person.into(),
			employee_status,
			title,
			store,
		)
		.await?;

		Ok(())
	}

	async fn create_job<'err, J, L, O>(
		hourly_rate: Money,
		year: Option<i32>,
		month: Option<u32>,
		day: Option<u32>,
		hour: Option<u32>,
		minute: Option<u32>,
		store: &Store,
	) -> DynResult<'err, ()>
	where
		J: JobAdapter,
		L: LocationAdapter + Send,
		O: OrganizationAdapter + Send,

		<J as JobAdapter>::Error: 'err,
		<L as LocationAdapter>::Error: 'err,
		<O as OrganizationAdapter>::Error: 'err,
	{
		let organization_views = input::util::organization::retrieve_views::<&str, L, O>(
			"Query the client `Organization` for this `Job`",
			false,
			store,
		)
		.await?;

		let client = input::select_one(&organization_views, "Select the client for this job")?;

		let objectives = input::edit_markdown("* List your objectives\n* All markdown syntax works")?;

		// [null]                               = current date and time
		// <year> <month> <day>                 = that day, midnight
		// <year> <month> <day> <hour> <minute> = that day and time
		let local_date_open = {
			let now = Local::now();

			let date = Local.ymd(
				year.unwrap_or_else(|| now.year()),
				month.unwrap_or_else(|| now.month()),
				day.unwrap_or_else(|| now.day()),
			);

			if year.is_some() && hour.is_none()
			{
				date.and_hms(0, 0, 0)
			}
			else
			{
				date.and_hms(
					hour.unwrap_or_else(|| now.hour()),
					minute.unwrap_or_else(|| now.minute()),
					0,
				)
			}
		};

		J::create(
			client.into(),
			local_date_open.into(),
			hourly_rate,
			objectives,
			store,
		)
		.await?;

		Ok(())
	}

	async fn create_location<'store, F, Fut, L>(
		create_inner: F,
		names: Vec<String>,
		store: &'store Store,
	) -> Result<(), <L as LocationAdapter>::Error>
	where
		F: Fn(Location, String, &'store Store) -> Fut,
		Fut: Future<Output = Result<Location, <L as LocationAdapter>::Error>>,
		L: LocationAdapter,
	{
		if let Some(name) = names.last()
		{
			let outer = L::create(name.clone(), store).await?;
			stream::iter(names.into_iter().rev().skip(1).map(Ok))
				.try_fold(outer, |outer, name| async {
					create_inner(outer, name, store).await
				})
				.await?;
		}

		Ok(())
	}

	async fn create_organization<'err, L, O>(name: String, store: &Store) -> DynResult<'err, ()>
	where
		L: LocationAdapter + Send,
		O: OrganizationAdapter,

		<L as LocationAdapter>::Error: 'err,
		<O as OrganizationAdapter>::Error: 'err,
	{
		let location_views = input::util::location::retrieve_views::<&str, L>(
			"Query the `Location` of this `Organization`",
			false,
			store,
		)
		.await?;

		let selected_view =
			input::select_one(&location_views, format!("Select a location for {}", name))?;

		O::create(selected_view.into(), name, store).await?;

		Ok(())
	}

	pub(super) async fn run<'config>(
		self,
		config: &'config Config<'_, '_>,
		store_name: String,
	) -> DynResult<'config, ()>
	{
		let store = config
			.get_store(&store_name)
			.expect("Storage name not known");

		match store.adapter
		{
			#[cfg(feature = "bincode")]
			Adapters::Bincode => match self
			{
				Self::Employee { title } =>
				{
					Self::create_employee::<
						BincodeEmployee,
						BincodeLocation,
						BincodeOrganization,
						BincodePerson,
					>(title, store)
					.await
				},

				Self::Job {
					currency,
					hourly_rate,
					year,
					month,
					day,
					hour,
					minute,
				} =>
				{
					Self::create_job::<BincodeJob, BincodeLocation, BincodeOrganization>(
						Money {
							amount:   hourly_rate,
							currency: currency.unwrap_or(config.invoices.default_currency),
						},
						year,
						month,
						day,
						hour,
						minute,
						store,
					)
					.await
				},

				Self::Location { names } =>
				{
					Self::create_location::<_, _, BincodeLocation>(
						|loc, name, store| async move {
							BincodeLocation {
								location: &loc,
								store,
							}
							.create_inner(name)
							.await
						},
						names,
						store,
					)
					.err_into()
					.await
				},

				Self::Organization { name } =>
				{
					Self::create_organization::<BincodeLocation, BincodeOrganization>(name, store).await
				},

				Self::Person { name } => BincodePerson::create(name, store)
					.err_into()
					.await
					.and(Ok(())),
			},

			_ => return Err(Error::FeatureNotFound(store.adapter).into()),
		}?;

		Ok(())
	}
}
