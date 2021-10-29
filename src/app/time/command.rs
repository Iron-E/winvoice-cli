mod display;

use std::borrow::Cow::Owned;

use clinvoice_adapter::data::{Deletable, EmployeeAdapter, JobAdapter, TimesheetAdapter};
use clinvoice_data::{
	chrono::{Duration, DurationRound, Utc},
	views::JobView,
	Employee,
	Id,
	Job,
};
use clinvoice_query as query;
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
		employee: &Employee,
		job: &Job,
	) -> Result<()>
	where
		Db: Database,
		TAdapter: Deletable<Db = Db> + TimesheetAdapter + Send,
		for<'c> &'c mut Db::Connection: Executor<'c, Database = Db>,
	{
		TAdapter::create(connection, employee, job)
			.await
			.and(Ok(()))
	}

	async fn stop<'err, Db, TAdapter>(
		connection: &Pool<Db>,
		default_employee_id: Option<Id>,
		job: &JobView,
	) -> DynResult<'err, ()>
	where
		Db: Database,
		TAdapter: Deletable<Db = Db> + TimesheetAdapter + Send,
		for<'c> &'c mut Db::Connection: Executor<'c, Database = Db>,
	{
		let mut timesheet = {
			let timesheets = TAdapter::retrieve_view(connection, &query::Timesheet {
				employee: query::Employee {
					id: if let Some(default) = default_employee_id
					{
						query::Match::EqualTo(Owned(default))
					}
					else
					{
						query::Match::Any
					},
					..Default::default()
				},
				time_end: query::Match::EqualTo(Owned(None)),
				..Default::default()
			})
			.await?;

			let selected = input::select_one(&timesheets, "Which `Timesheet` are you working on?")?;
			selected.to_owned()
		};

		timesheet.work_notes = input::edit_markdown(&timesheet.work_notes)?;

		input::util::expense::menu(&mut timesheet.expenses, job.invoice.hourly_rate.currency)?;

		// Stop time on the `Job` AFTER requiring users to enter information. Users shouldn't enter things for free ;)
		let increment = Duration::from_std(job.increment)?;
		timesheet.time_begin = timesheet.time_begin.duration_round(increment)?;
		timesheet.time_end = Some(Utc::now().duration_round(increment)?);

		TAdapter::update(connection, timesheet.into()).await?;

		Ok(())
	}

	pub async fn run<'err, Db, EAdapter, JAdapter, TAdapter>(
		&self,
		connection: Pool<Db>,
		default_employee_id: Option<Id>,
	) -> DynResult<'err, ()>
	where
		Db: Database,
		EAdapter: Deletable<Db = Db> + EmployeeAdapter + Send,
		JAdapter: Deletable<Db = Db> + JobAdapter + Send,
		TAdapter: Deletable<Db = Db> + TimesheetAdapter + Send,
		for<'c> &'c mut Db::Connection: Executor<'c, Database = Db>,
	{
		let job_results_view: Vec<_> = input::util::job::retrieve_view::<&str, _, JAdapter>(
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
			format!("Select the job to {} working on", self),
		)?;

		match self
		{
			Self::Start =>
			{
				let results_view = input::util::employee::retrieve_view::<&str, _, EAdapter>(
					&connection,
					default_employee_id,
					"Query the `Employee` who will be doing the work",
					true,
				)
				.await?;

				let selected_employee = input::select_one(
					&results_view,
					format!("Select the `Employee` who is doing the work"),
				)?;

				Self::start::<_, TAdapter>(
					&connection,
					&selected_employee.into(),
					&selected_job.into(),
				)
				.await?;
			},

			Self::Stop =>
			{
				Self::stop::<_, TAdapter>(&connection, default_employee_id, &selected_job).await?
			},
		};

		Ok(())
	}
}
