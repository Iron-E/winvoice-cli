mod display;

use
{
	std::{borrow::Cow::Borrowed, cmp::Ordering, error::Error},

	super::QUERY_PROMPT,
	crate::{Config, DynResult, input, StructOpt},

	clinvoice_adapter::
	{
		Adapters, Error as AdapterError,
		data::{EmployeeAdapter, Error as DataError, JobAdapter, query, Updatable},
	},
	clinvoice_data::
	{
		chrono::{Duration, DurationRound, Utc},
		Job,
		views::{EmployeeView, JobView},
	},
};

#[cfg(feature="bincode")]
use clinvoice_adapter_bincode::data::{BincodeEmployee, BincodeJob, BincodeLocation, BincodeOrganization, BincodePerson};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructOpt)]
#[structopt(about="Time information that was recorded with CLInvoice")]
pub(super) struct Time
{
	#[structopt(help="Do work as the default `Employee`, as specified in your configuration", long, short)]
	pub default: bool,

	#[structopt(subcommand)]
	pub command: TimeCommand,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructOpt)]
pub(super) enum TimeCommand
{
	#[structopt(about="Start working on a `Job`")]
	Start,

	#[structopt(about="Stop working on a `Job`")]
	Stop,
}

impl Time
{
	fn start<'err, E>(employee: EmployeeView, job: JobView, update_job: impl Fn(Job) -> Result<(), E>)
		-> DynResult<'err, ()>
	where
		E : 'err + Error,
	{
		let mut job: Job = job.into();
		job.start_timesheet(employee.id);
		update_job(job).map_err(|e| e.into())
	}

	fn stop<'err, E>(config: &Config, mut job: JobView, update_job: impl Fn(JobView) -> Result<(), E>)
		-> DynResult<'err, ()>
	where
		E : 'err + Error,
	{
		let mut selected = input::select_one(
			&job.timesheets.iter().filter(|t| t.time_end.is_none()).collect::<Vec<_>>(),
			"Which `Timesheet` are you working on?",
		)?.clone();

		// We want to remove the `selected` timesheet from the set of timseheets; for now.
		job.timesheets.remove(job.timesheets.iter().enumerate().fold(0, |i, enumeration|
			if &selected == enumeration.1 { enumeration.0 }
			else { i }
		));

		// TODO: enter expenses, work notes, etc.

		// Stop time on the `Job` AFTER requiring users to enter information. Users shouldn't enter things for free ;)
		let interval = Duration::from_std(config.timesheets.interval)?;
		selected.time_begin = selected.time_begin.duration_trunc(interval)?;
		selected.time_end = Some(Utc::now().duration_trunc(interval)?);

		// Now that `selected` is done being ammended, we can insert it back.
		job.timesheets.push(selected);
		job.timesheets.sort_by(|t1, t2|
			if t1.time_begin == t2.time_begin
			{
				t1.time_begin.cmp(&t2.time_begin)
			}
			else
			{
				t1.time_end.map(|time|
					// If they both have a time, compare it. Otherwise, `t1` has ended and `t2` has not, so
					// `t1` is less than `t2`.
					t2.time_end.map(|other_time| time.cmp(&other_time)).unwrap_or(Ordering::Less)
				).unwrap_or_else(||
					// If `t1` has not ended, but `t2` has, then `t1` is greater. Otherwise, if neither has
					// ended, then they are equal.
					t2.time_end.and(Some(Ordering::Greater)).unwrap_or(Ordering::Equal)
				)
			}
		);

		update_job(job).map_err(|e| e.into())
	}

	/// # Summary
	///
	/// Execute the constructed command.
	pub(super) fn run<'err>(self, config: &Config, store_name: String) -> DynResult<'err, ()>
	{
		let store = config.get_store(&store_name).expect("Storage name not known");

		macro_rules! retrieve
		{
			($emp: ident, $job: ident, $loc: ident, $org: ident, $per: ident) =>
			{{
				let job_query: query::Job = input::edit_default(String::from(QUERY_PROMPT) + "jobs")?;

				let job_results = $job::retrieve(&job_query, &store)?;
				let job_results_view = job_results.into_iter().map(|j|
					$job::into_view::<$emp, $loc, $org, $per>(j, &store)
				).filter_map(|result| match result
				{
					Ok(t) => match job_query.matches_view(&t)
					{
						Ok(b) if b => Some(Ok(t)),
						Err(e) => Some(Err(DataError::from(e).into())),
						_ => None,
					},
					Err(e) => Some(Err(e)),
				}).collect::<Result<Vec<_>, _>>()?;

				let selected_job = input::select_one(&job_results_view, format!("Select the job to {} working on", self.command))?;

				match self.command
				{
					TimeCommand::Start =>
					{
						let query = if self.default
						{
							query::Employee
							{
								id: query::Match::EqualTo(Borrowed(&config.employees.default_id)),
								..Default::default()
							}
						}
						else
						{
							input::edit_default(String::from(QUERY_PROMPT) + "employees")?
						};

						let results = $emp::retrieve(&query, &store)?;
						let results_view = results.into_iter().map(|j|
							$emp::into_view::<$loc, $org, $per>(j, &store)
						).filter_map(|result| match result
						{
							Ok(t) => match query.matches_view(&t)
							{
								Ok(b) if b => Some(Ok(t)),
								Err(e) => Some(Err(DataError::from(e).into())),
								_ => None,
							},
							Err(e) => Some(Err(e)),
						}).collect::<Result<Vec<_>, _>>()?;

						let selected = input::select_one(&results_view, format!("Select the `Employee` who is doing the work"))?;

						Self::start(selected, selected_job, |j| $job {job: &j, store}.update())?
					},

					TimeCommand::Stop => Self::stop(config, selected_job, |j| $job {job: &(j.into()), store}.update())?,
				};
			}};
		}

		match store.adapter
		{
			#[cfg(feature="bincode")]
			Adapters::Bincode => retrieve!(BincodeEmployee, BincodeJob, BincodeLocation, BincodeOrganization, BincodePerson),

			_ => return Err(AdapterError::FeatureNotFound(store.adapter).into()),
		};

		Ok(())
	}
}

