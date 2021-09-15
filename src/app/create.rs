use clinvoice_adapter::{
	data::{EmployeeAdapter, JobAdapter, LocationAdapter, OrganizationAdapter, PersonAdapter},
	Adapters,
	Error as FeatureNotFoundError,
	Store,
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

use crate::{input, DynResult, StructOpt};

#[cfg(feature="postgres")]
use clinvoice_adapter_postgres::data::{PostgresEmployee, PostgresJob, PostgresLocation, PostgresOrganization, PostgresPerson};

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
	async fn create_employee<'a, E, L, O, Pr, Pl>(title: String, pool: &'a Pl) -> DynResult<'a, ()>
	where
		E: EmployeeAdapter<Pool = &'a Pl>,
		L: LocationAdapter<Pool = &'a Pl> + Send,
		O: OrganizationAdapter<Pool = &'a Pl> + Send,
		Pr: PersonAdapter<Pool = &'a Pl>,

		<E as EmployeeAdapter>::Error: 'a,
		<L as LocationAdapter>::Error: 'a,
		<O as OrganizationAdapter>::Error: 'a,
		<Pr as PersonAdapter>::Error: 'a,
	{
		let organization_views = input::util::organization::retrieve_view::<&str, O, _>(
			"Query the `Organization` where this `Employee` works",
			false,
			pool,
		)
		.await?;

		let organization = input::select_one(
			&organization_views,
			"Which organization does this employee work at?",
		)?;

		let person_views = input::util::person::retrieve_view::<&str, Pr, _>(
			"Query the `Person` who this `Employee` is",
			true,
			pool,
		)
		.await?;

		let person = input::select_one(&person_views, "Which `Person` is this `Employee`?")?;

		let contact_info = input::util::contact::menu::<L, _>(pool).await?;
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
			pool,
		)
		.await?;

		Ok(())
	}

	async fn create_job<'a, J, O, P>(
		hourly_rate: Money,
		year: Option<i32>,
		month: Option<u32>,
		day: Option<u32>,
		hour: Option<u32>,
		minute: Option<u32>,
		pool: &'a P,
	) -> DynResult<'a, ()>
	where
		J: JobAdapter<Pool = &'a P>,
		O: OrganizationAdapter<Pool = &'a P> + Send,

		<J as JobAdapter>::Error: 'a,
		<O as OrganizationAdapter>::Error: 'a,
	{
		let organization_views = input::util::organization::retrieve_view::<&str, O, _>(
			"Query the client `Organization` for this `Job`",
			false,
			pool,
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
			pool,
		)
		.await?;

		Ok(())
	}

	async fn create_location<'a, F, Fut, L, P>(
		create_inner: F,
		names: Vec<String>,
		pool: &'a P,
	) -> Result<(), <L as LocationAdapter>::Error>
	where
		F: Fn(Location, String, &'a P) -> Fut,
		Fut: Future<Output = Result<Location, <L as LocationAdapter>::Error>>,
		L: LocationAdapter<Pool = &'a P>,
	{
		if let Some(name) = names.last()
		{
			let outer = L::create(name.clone(), pool).await?;
			stream::iter(names.into_iter().rev().skip(1).map(Ok))
				.try_fold(outer, |outer, name| async {
					create_inner(outer, name, pool).await
				})
				.await?;
		}

		Ok(())
	}

	async fn create_organization<'a, L, O, P>(name: String, pool: &'a P) -> DynResult<'a, ()>
	where
		L: LocationAdapter<Pool = &'a P> + Send,
		O: OrganizationAdapter<Pool = &'a P>,

		<L as LocationAdapter>::Error: 'a,
		<O as OrganizationAdapter>::Error: 'a,
	{
		let location_views = input::util::location::retrieve_view::<&str, L, _>(
			"Query the `Location` of this `Organization`",
			false,
			pool,
		)
		.await?;

		let selected_view =
			input::select_one(&location_views, format!("Select a location for {}", name))?;

		O::create(selected_view.into(), name, pool).await?;

		Ok(())
	}

	pub(super) async fn run<'err>(
		self,
		default_currency: Currency,
		store: &Store,
	) -> DynResult<'err, ()>
	{
		match store.adapter
		{
			#[cfg(feature="postgres")]
			Adapters::Postgres =>
			{
				let pool = sqlx::PgPool::connect_lazy(&store.url)?;

				match self
				{
					Self::Employee { title } =>
					{
						Self::create_employee::<
							PostgresEmployee,
							PostgresLocation,
							PostgresOrganization,
							PostgresPerson,
							_,
						>(title, &pool)
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
						Self::create_job::<PostgresJob, PostgresOrganization, _>(
							Money {
								amount:   hourly_rate,
								currency: currency.unwrap_or(default_currency),
							},
							year,
							month,
							day,
							hour,
							minute,
							&pool,
						)
						.await
					},

					Self::Location { names } =>
					{
						Self::create_location::<_, _, PostgresLocation, _>(
							|loc, name, store| async move {
								PostgresLocation {
									location: &loc,
									pool: &pool,
								}
								.create_inner(name)
								.await
							},
							names,
							&pool,
						)
						.err_into()
						.await
					},

					Self::Organization { name } =>
					{
						Self::create_organization::<PostgresLocation, PostgresOrganization, _>(name, &pool).await
					},

					Self::Person { name } => PostgresPerson::create(name, &pool)
						.err_into()
						.await
						.and(Ok(())),
				}
			},

			_ => return Err(FeatureNotFoundError(store.adapter).into()),
		}?;

		Ok(())
	}
}
