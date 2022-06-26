mod display;

use clinvoice_adapter::{
	schema::{EmployeeAdapter, ExpensesAdapter, JobAdapter, TimesheetAdapter},
	Deletable,
};
use clinvoice_match::{Match, MatchEmployee, MatchTimesheet};
use clinvoice_schema::{
	chrono::{Duration, DurationRound, Utc},
	Employee,
	Id,
	Job,
};
use futures::TryFutureExt;
use sqlx::{Database, Executor, Pool, Result};
use structopt::StructOpt;

use crate::{input, DynResult};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructOpt)]
pub enum Command
{
	#[structopt(about = "Start working on a `Job`")]
	Start,

	#[structopt(about = "Stop working on a `Job`")]
	Stop,
}

impl Command
{
	async fn start<'err, Db, TAdapter>(
		connection: &Pool<Db>,
		employee: Employee,
		job: Job,
	) -> Result<()>
	where
		Db: Database,
		TAdapter: Deletable<Db = Db> + TimesheetAdapter,
		for<'c> &'c mut Db::Connection: Executor<'c, Database = Db>,
	{
		TAdapter::create(connection, employee, Vec::new(), job, Utc::now(), None).await?;
		Ok(())
	}

	async fn stop<Db, TAdapter, XAdapter>(
		connection: &Pool<Db>,
		default_employee_id: Option<Id>,
		job: Job,
	) -> DynResult<()>
	where
		Db: Database,
		TAdapter: Deletable<Db = Db> + TimesheetAdapter,
		XAdapter: Deletable<Db = Db> + ExpensesAdapter,
		for<'c> &'c mut Db::Connection: Executor<'c, Database = Db>,
	{
		let mut timesheet = {
			let timesheets = TAdapter::retrieve(connection, &MatchTimesheet {
				employee: MatchEmployee {
					id: if let Some(default) = default_employee_id
					{
						default.into()
					}
					else
					{
						Match::Any
					},
					..Default::default()
				},
				time_end: None.into(),
				..Default::default()
			})
			.await?;

			let selected = input::select_one(&timesheets, "Which `Timesheet` are you working on?")?;
			selected.to_owned()
		};

		timesheet.work_notes = input::edit_markdown(&timesheet.work_notes)?;

		input::util::expense::menu::<_, XAdapter>(
			connection,
			&mut timesheet.expenses,
			job.invoice.hourly_rate.currency,
			job.id,
		)
		.await?;

		// Stop time on the `Job` AFTER requiring users to enter information. Users shouldn't enter things for free ;)
		let increment = Duration::from_std(job.increment)?;
		timesheet.time_begin = timesheet.time_begin.duration_round(increment)?;
		timesheet.time_end = Some(Utc::now().duration_round(increment)?);

		connection
			.begin()
			.and_then(|mut transaction| async {
				TAdapter::update(&mut transaction, [&timesheet].into_iter()).await?;
				transaction.commit().await
			})
			.await?;

		Ok(())
	}

	pub async fn run<Db, EAdapter, JAdapter, TAdapter, XAdapter>(
		&self,
		connection: Pool<Db>,
		default_employee_id: Option<Id>,
	) -> DynResult<()>
	where
		Db: Database,
		EAdapter: Deletable<Db = Db> + EmployeeAdapter,
		JAdapter: Deletable<Db = Db> + JobAdapter,
		TAdapter: Deletable<Db = Db> + TimesheetAdapter,
		XAdapter: Deletable<Db = Db> + ExpensesAdapter,
		for<'c> &'c mut Db::Connection: Executor<'c, Database = Db>,
	{
		let job_results_view: Vec<_> = input::util::job::retrieve::<&str, _, JAdapter>(
			&connection,
			"Query the `Job` which you are working on",
			false,
		)
		.await?
		.into_iter()
		.filter(|j| j.date_close.is_none())
		.collect();

		let selected_job = input::select_one(
			&job_results_view,
			format!("Select the job to {self} working on"),
		)?;

		match self
		{
			Self::Start =>
			{
				let results_view = input::util::employee::retrieve::<&str, _, EAdapter>(
					&connection,
					default_employee_id,
					"Query the `Employee` who will be doing the work",
					true,
				)
				.await?;

				let selected_employee = input::select_one(
					&results_view,
					"Select the `Employee` who is doing the work".to_string(),
				)?;

				Self::start::<_, TAdapter>(&connection, selected_employee, selected_job).await?;
			},

			Self::Stop =>
			{
				Self::stop::<_, TAdapter, XAdapter>(&connection, default_employee_id, selected_job)
					.await?
			},
		};

		Ok(())
	}
}
