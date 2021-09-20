mod display;

use std::cmp::Ordering;

use clinvoice_adapter::data::{Deletable, EmployeeAdapter, JobAdapter};
use clinvoice_data::{Id, chrono::{Duration, DurationRound, Utc}, finance::Currency, views::{EmployeeView, JobView, TimesheetView}};
use sqlx::{Database, Pool};
use structopt::StructOpt;
use core::time::Duration as StdDuration;

use crate::{DynResult, input};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructOpt)]
pub(super) enum Command
{
	#[structopt(about = "Start working on a `Job`")]
	Start,

	#[structopt(about = "Stop working on a `Job`")]
	Stop,
}

impl Command
{
	fn start(employee: EmployeeView, job: &mut JobView)
	{
		job.timesheets.push(TimesheetView {
			employee,
			expenses: Vec::new(),
			time_begin: Utc::now(),
			time_end: None,
			work_notes: "* Work which was done goes here\n* Supports markdown formatting".into(),
		})
	}

	fn stop<'err>(
		default_currency: Currency,
		default_employee_id: Option<Id>,
		default_timesheet_interval: StdDuration,
		job: &mut JobView
	) -> DynResult<'err, ()>
	{
		let index = {
			let timesheets: Vec<_> = job
				.timesheets
				.iter()
				.filter(|t| {
					let is_active = t.time_end.is_none();
					if let Some(default) = default_employee_id
					{
						is_active && t.employee.id == default
					}
					else
					{
						is_active
					}
				})
				.collect();

			if timesheets.is_empty()
			{
				return Err(input::Error::NoData(
					format!("active `{}`s", stringify!(Timesheet))
				).into());
			}

			let selected = input::select_one(&timesheets, "Which `Timesheet` are you working on?")?;

			job.timesheets.iter().enumerate().fold(0, |i, enumeration| {
				if selected == enumeration.1
				{
					enumeration.0
				}
				else
				{
					i
				}
			})
		};

		job.timesheets[index].work_notes = input::edit_markdown(&job.timesheets[index].work_notes)?;

		input::util::expense::menu(
			&mut job.timesheets[index].expenses,
			default_currency,
		)?;

		// Stop time on the `Job` AFTER requiring users to enter information. Users shouldn't enter things for free ;)
		let interval = Duration::from_std(default_timesheet_interval)?;
		job.timesheets[index].time_begin =
			job.timesheets[index].time_begin.duration_trunc(interval)?;
		job.timesheets[index].time_end = Some(Utc::now().duration_trunc(interval)?);

		// Now that `job.timesheets[index]` is done being ammended, we can resort the timesheets.
		job.timesheets.sort_by(|t1, t2| {
			if t1.time_begin != t2.time_begin
			{
				t1.time_begin.cmp(&t2.time_begin)
			}
			else
			{
				t1.time_end
					.map(|time|
					// If they both have a time, compare it. Otherwise, `t1` has ended and `t2` has not, so
					// `t1` is less than `t2`.
					t2.time_end.map(|other_time| time.cmp(&other_time)).unwrap_or(Ordering::Less))
					.unwrap_or_else(||
					// If `t1` has not ended, but `t2` has, then `t1` is greater. Otherwise, if neither has
					// ended, then they are equal.
					t2.time_end.and(Some(Ordering::Greater)).unwrap_or(Ordering::Equal))
			}
		});

		Ok(())
	}

	pub(super) async fn run<'err, Db, EAdapter, JAdapter>(
		&self,
		connection: Pool<Db>,
		default_currency: Currency,
		default_employee_id: Option<Id>,
		default_timesheet_interval: StdDuration,
	) -> DynResult<'err, ()> where
		Db: Database,
		EAdapter : Deletable<Db = Db> + EmployeeAdapter + Send,
		JAdapter : Deletable<Db = Db> + JobAdapter + Send,
	{
		let job_results_view: Vec<_> =
			input::util::job::retrieve_view::<&str, _, JAdapter>(
				&connection,
				"Query the `Job` which you are working on",
				false,
			)
			.await?
			.into_iter()
			.filter(|j| j.date_close.is_none())
			.collect();

		let mut selected_job = input::select_one(
			&job_results_view,
			format!("Select the job to {} working on", self),
		)?;

		match self
		{
			Self::Start =>
			{
				let results_view =
					input::util::employee::retrieve_view::<&str, _, EAdapter>(
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

				Self::start(selected_employee, &mut selected_job)
			},

			Self::Stop => Self::stop(
				default_currency,
				default_employee_id,
				default_timesheet_interval,
				&mut selected_job,
			)?,
		};

		JAdapter::update(&connection, &selected_job.into()).await?;

		Ok(())
	}
}
