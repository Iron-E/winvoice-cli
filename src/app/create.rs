use clinvoice_adapter::{Adapters, Error as FeatureNotFoundError, Store, data::{Deletable, EmployeeAdapter, JobAdapter, LocationAdapter, OrganizationAdapter, PersonAdapter}};
use clinvoice_data::{
	chrono::{Datelike, Local, TimeZone, Timelike},
	finance::{Currency, Decimal, Money},
	EmployeeStatus,
};
use futures::{
	stream::{self, TryStreamExt},
	TryFutureExt,
};
use sqlx::{Database, Pool};

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
	async fn create_employee<'err, Db, EAdapter, LAdapter, OAdapter, PAdapter>(connection: &Pool<Db>, title: String) -> DynResult<'err, ()>
	where
		Db: Database,
		EAdapter: Deletable<Db = Db> + EmployeeAdapter + Send,
		LAdapter: Deletable<Db = Db> + LocationAdapter + Send,
		OAdapter: Deletable<Db = Db> + OrganizationAdapter + Send,
		PAdapter: Deletable<Db = Db> + PersonAdapter + Send,
		<EAdapter as Deletable>::Error: 'err,
	{
		let organization_views = input::util::organization::retrieve_view::<&str, _, OAdapter>(
			connection,
			"Query the `Organization` where this `Employee` works",
			false,
		)
		.await?;

		let organization = input::select_one(
			&organization_views,
			"Which organization does this employee work at?",
		)?;

		let person_views = input::util::person::retrieve_view::<&str, _, PAdapter>(
			connection,
			"Query the `Person` who this `Employee` is",
			true,
		)
		.await?;

		let person = input::select_one(&person_views, "Which `Person` is this `Employee`?")?;

		let contact_info = input::util::contact::menu::<_, LAdapter>(connection).await?;
		let employee_status = input::select_one(
			&[
				EmployeeStatus::Employed,
				EmployeeStatus::NotEmployed,
				EmployeeStatus::Representative,
			],
			"What is the status of the employee?",
		)?;

		EAdapter::create(
			connection,
			contact_info
				.into_iter()
				.map(|(label, contact)| (label, contact.into()))
				.collect(),
			organization.into(),
			person.into(),
			employee_status,
			title,
		)
		.await?;

		Ok(())
	}

	async fn create_job<'err, Db, JAdapter, OAdapter>(
		connection: &Pool<Db>,
		hourly_rate: Money,
		year: Option<i32>,
		month: Option<u32>,
		day: Option<u32>,
		hour: Option<u32>,
		minute: Option<u32>,
	) -> DynResult<'err, ()>
	where
		Db: Database,
		JAdapter: Deletable<Db = Db> + JobAdapter,
		OAdapter: Deletable<Db = Db> + OrganizationAdapter + Send,
	{
		let organization_views = input::util::organization::retrieve_view::<&str, _, OAdapter>(
			connection,
			"Query the client `Organization` for this `Job`",
			false,
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

		JAdapter::create(
			connection,
			client.into(),
			local_date_open.into(),
			hourly_rate,
			objectives,
		)
		.await?;

		Ok(())
	}

	async fn create_location<'a, Db, LAdapter>(
		connection: &Pool<Db>,
		names: Vec<String>,
	) -> Result<(), <LAdapter as Deletable>::Error>
	where
		Db: Database,
		LAdapter: Deletable<Db = Db> + LocationAdapter,
	{
		if let Some(name) = names.last()
		{
			let outer = LAdapter::create(connection, name.clone()).await?;
			stream::iter(names.into_iter().rev().skip(1).map(Ok))
				.try_fold(outer, |outer, name| async {
					LAdapter::create_inner(connection, &outer, name).await
				})
				.await?;
		}

		Ok(())
	}

	async fn create_organization<'a, Db, LAdapter, OAdapter>(connection: &Pool<Db>, name: String) -> DynResult<'a, ()>
	where
		Db: Database,
		LAdapter: Deletable<Db = Db> + LocationAdapter + Send,
		OAdapter: Deletable<Db = Db> + OrganizationAdapter,
	{
		let location_views = input::util::location::retrieve_view::<&str, _, LAdapter>(
			connection,
			"Query the `Location` of this `Organization`",
			false,
		)
		.await?;

		let selected_view =
			input::select_one(&location_views, format!("Select a location for {}", name))?;

		OAdapter::create(connection, selected_view.into(), name).await?;

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
							_,
							PostgresEmployee,
							PostgresLocation,
							PostgresOrganization,
							PostgresPerson,
						>(&pool, title)
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
						Self::create_job::<_, PostgresJob, PostgresOrganization,>(
							&pool,
							Money {
								amount:   hourly_rate,
								currency: currency.unwrap_or(default_currency),
							},
							year,
							month,
							day,
							hour,
							minute,
						)
						.await
					},

					Self::Location { names } =>
					{
						Self::create_location::<_, PostgresLocation>(&pool, names)
						.err_into()
						.await
					},

					Self::Organization { name } =>
					{
						Self::create_organization::<_, PostgresLocation, PostgresOrganization>(&pool, name).await
					},

					Self::Person { name } => PostgresPerson::create(&pool, name)
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
