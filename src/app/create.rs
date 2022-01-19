use core::time::Duration as StdDuration;

use clinvoice_adapter::{
	schema::{EmployeeAdapter, JobAdapter, LocationAdapter, OrganizationAdapter, PersonAdapter},
	Adapters,
	Deletable,
	Error as FeatureNotFoundError,
	Store,
};
use clinvoice_schema::{
	chrono::{Local, TimeZone},
	Currency,
	Decimal,
	Money,
};
use futures::{
	stream::{self, TryStreamExt},
	TryFutureExt,
};
use humantime::Duration;
use sqlx::{Database, Executor, Pool, Result};
use structopt::StructOpt;
#[cfg(feature = "postgres")]
use {
	clinvoice_adapter_postgres::schema::{PgEmployee, PgJob, PgLocation, PgOrganization, PgPerson},
	sqlx::PgPool,
};

use crate::{input, DynResult};

#[derive(Clone, Debug, Eq, Hash, PartialEq, StructOpt)]
#[structopt(about = "Record information information with CLInvoice")]
pub enum Create
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

		#[structopt(
			help = "The increment that time in `Timesheet`s is rounded to when running `clinvoice \
			        time stop`",
			long,
			short
		)]
		increment: Option<Duration>,

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
	async fn create<'err, Db, EAdapter, JAdapter, LAdapter, OAdapter, PAdapter>(
		self,
		connection: Pool<Db>,
		default_currency: Currency,
		default_increment: StdDuration,
	) -> DynResult<'err, ()>
	where
		Db: Database,
		EAdapter: Deletable<Db = Db> + EmployeeAdapter + Send,
		JAdapter: Deletable<Db = Db> + JobAdapter + Send,
		LAdapter: Deletable<Db = Db> + LocationAdapter + Send,
		OAdapter: Deletable<Db = Db> + OrganizationAdapter + Send,
		PAdapter: Deletable<Db = Db> + PersonAdapter + Send,
		for<'c> &'c mut Db::Connection: Executor<'c, Database = Db>,
	{
		match self
		{
			Self::Employee { title } =>
			{
				Self::create_employee::<_, EAdapter, LAdapter, OAdapter, PAdapter>(&connection, title)
					.await
			},

			Self::Job {
				currency,
				increment,
				hourly_rate,
				year,
				month,
				day,
				hour,
				minute,
			} =>
			{
				Self::create_job::<_, JAdapter, OAdapter>(
					&connection,
					Money {
						amount: hourly_rate,
						currency: currency.unwrap_or(default_currency),
					},
					increment.map(Duration::into).unwrap_or(default_increment),
					year.map(|y| {
						(
							y,
							month.expect("`month` requires `year`"),
							day.expect("`day` requires `month`"),
							hour.map(|h| (h, minute.expect("`hour` requires `minute`"))),
						)
					}),
				)
				.await
			},

			Self::Location { names } =>
			{
				Self::create_location::<_, LAdapter>(&connection, names)
					.err_into()
					.await
			},

			Self::Organization { name } =>
			{
				Self::create_organization::<_, LAdapter, OAdapter>(&connection, name).await
			},

			Self::Person { name } => PAdapter::create(&connection, name)
				.err_into()
				.await
				.and(Ok(())),
		}
	}

	async fn create_employee<'err, Db, EAdapter, LAdapter, OAdapter, PAdapter>(
		connection: &Pool<Db>,
		title: String,
	) -> DynResult<'err, ()>
	where
		Db: Database,
		EAdapter: Deletable<Db = Db> + EmployeeAdapter + Send,
		LAdapter: Deletable<Db = Db> + LocationAdapter + Send,
		OAdapter: Deletable<Db = Db> + OrganizationAdapter + Send,
		PAdapter: Deletable<Db = Db> + PersonAdapter + Send,
		for<'c> &'c mut Db::Connection: Executor<'c, Database = Db>,
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
		let employee_status = input::text(None, "What is the status of the employee?")?;

		EAdapter::create(
			connection,
			contact_info
				.into_iter()
				.map(|(label, contact)| (label, contact.into()))
				.collect(),
			&organization.into(),
			&person.into(),
			employee_status,
			title,
		)
		.err_into()
		.await
		.and(Ok(()))
	}

	#[allow(clippy::type_complexity)]
	async fn create_job<'err, Db, JAdapter, OAdapter>(
		connection: &Pool<Db>,
		hourly_rate: Money,
		increment: StdDuration,
		year_month_day_hour_minute: Option<(i32, u32, u32, Option<(u32, u32)>)>,
	) -> DynResult<'err, ()>
	where
		Db: Database,
		JAdapter: Deletable<Db = Db> + JobAdapter,
		OAdapter: Deletable<Db = Db> + OrganizationAdapter + Send,
		for<'c> &'c mut Db::Connection: Executor<'c, Database = Db>,
	{
		let organization_views = input::util::organization::retrieve_view::<&str, _, OAdapter>(
			connection,
			"Query the client `Organization` for this `Job`",
			false,
		)
		.await?;

		let client = input::select_one(&organization_views, "Select the client for this job")?;

		let objectives = input::edit_markdown("* List your objectives\n* All markdown syntax works")?;

		// [null]                            = current date and time
		// <year><month><day>                = that day, midnight
		// <year><month><day> <hour><minute> = that day and time
		let local_date_open = year_month_day_hour_minute
			.map(|(year, month, day, hour_minute)| {
				let (hour, minute) = hour_minute.unwrap_or((0, 0));
				Local.ymd(year, month, day).and_hms(hour, minute, 0)
			})
			.unwrap_or_else(Local::now);

		JAdapter::create(
			connection,
			&client.into(),
			local_date_open.into(),
			hourly_rate,
			increment,
			objectives,
		)
		.err_into()
		.await
		.and(Ok(()))
	}

	async fn create_location<Db, LAdapter>(connection: &Pool<Db>, names: Vec<String>) -> Result<()>
	where
		Db: Database,
		LAdapter: Deletable<Db = Db> + LocationAdapter,
		for<'c> &'c mut Db::Connection: Executor<'c, Database = Db>,
	{
		if let Some(name) = names.last()
		{
			let outer = LAdapter::create(connection, name.clone()).await?;
			stream::iter(names.into_iter().rev().skip(1).map(Ok))
				.try_fold(outer, |outer, name| async move {
					LAdapter::create_inner(connection, &outer, name).await
				})
				.await?;
		}

		Ok(())
	}

	async fn create_organization<'err, Db, LAdapter, OAdapter>(
		connection: &Pool<Db>,
		name: String,
	) -> DynResult<'err, ()>
	where
		Db: Database,
		LAdapter: Deletable<Db = Db> + LocationAdapter + Send,
		OAdapter: Deletable<Db = Db> + OrganizationAdapter,
		for<'c> &'c mut Db::Connection: Executor<'c, Database = Db>,
	{
		let location_views = input::util::location::retrieve_view::<&str, _, LAdapter>(
			connection,
			"Query the `Location` of this `Organization`",
			false,
		)
		.await?;

		let selected_view =
			input::select_one(&location_views, format!("Select a location for {name}"))?;

		OAdapter::create(connection, &selected_view.into(), name)
			.err_into()
			.await
			.and(Ok(()))
	}

	pub async fn run<'err>(
		self,
		default_currency: Currency,
		default_increment: StdDuration,
		store: &Store,
	) -> DynResult<'err, ()>
	{
		match store.adapter
		{
			#[cfg(feature = "postgres")]
			Adapters::Postgres =>
			{
				let pool = PgPool::connect_lazy(&store.url)?;
				self
					.create::<_, PgEmployee, PgJob, PgLocation, PgOrganization, PgPerson>(
						pool,
						default_currency,
						default_increment,
					)
					.await
			},

			// NOTE: this is allowed because there may be additional adapters added later, and I want
			//       to define this behavior now.
			#[allow(unreachable_patterns)]
			_ => Err(FeatureNotFoundError(store.adapter).into()),
		}
	}
}
