mod display;

use std::cmp::Ordering;

use clinvoice_adapter::{
	data::{
		Error as DataError,
		Updatable,
	},
	Adapters,
	Error as AdapterError,
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
	chrono::{
		Duration,
		DurationRound,
		Utc,
	},
	views::{
		EmployeeView,
		JobView,
		TimesheetView,
	},
};

use crate::{
	input,
	Config,
	DynResult,
	StructOpt,
};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructOpt)]
#[structopt(about = "Time information that was recorded with CLInvoice")]
pub(super) struct Time
{
	#[structopt(
		help = "Do work as the default `Employee`, as specified in your configuration",
		long,
		short
	)]
	pub default: bool,

	#[structopt(subcommand)]
	pub command: TimeCommand,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructOpt)]
pub(super) enum TimeCommand
{
	#[structopt(about = "Start working on a `Job`")]
	Start,

	#[structopt(about = "Stop working on a `Job`")]
	Stop,
}

impl Time
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

	fn stop<'err>(config: &Config, default: bool, job: &mut JobView) -> DynResult<'err, ()>
	{
		let index = {
			let timesheets: Vec<_> = job
				.timesheets
				.iter()
				.filter(|t| {
					let is_active = t.time_end.is_none();
					if !default
					{
						is_active
					}
					else
					{
						is_active && t.employee.id == config.employees.default_id
					}
				})
				.collect();

			if timesheets.is_empty()
			{
				return Err(DataError::NoData(format!("active `{}`s", stringify!(Timesheet))).into());
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
			config.invoices.default_currency,
		)?;

		// Stop time on the `Job` AFTER requiring users to enter information. Users shouldn't enter things for free ;)
		let interval = Duration::from_std(config.timesheets.interval)?;
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

	/// # Summary
	///
	/// Execute the constructed command.
	pub(super) fn run<'err>(self, config: &Config, store_name: String) -> DynResult<'err, ()>
	{
		let store = config
			.get_store(&store_name)
			.expect("Storage name not known");

		macro_rules! retrieve {
			($emp:ident, $job:ident, $loc:ident, $org:ident, $per:ident) => {{
				let job_results_view: Vec<_> =
					input::util::job::retrieve_views::<&str, $emp, $job, $loc, $org, $per>(
						"Query the `Job` which you are working on",
						false,
						store,
					)?
					.into_iter()
					.filter(|j| j.date_close.is_none())
					.collect();

				let mut selected_job = input::select_one(
					&job_results_view,
					format!("Select the job to {} working on", self.command),
				)?;

				match self.command
				{
					TimeCommand::Start =>
					{
						let results_view =
							input::util::employee::retrieve_views::<&str, $emp, $loc, $org, $per>(
								if self.default
								{
									Some(config.employees.default_id)
								}
								else
								{
									None
								},
								"Query the `Employee` who will be doing the work",
								true,
								store,
							)?;

						let selected = input::select_one(
							&results_view,
							format!("Select the `Employee` who is doing the work"),
						)?;

						Self::start(selected, &mut selected_job)
					},

					TimeCommand::Stop => Self::stop(config, self.default, &mut selected_job)?,
				};

				$job {
					job: &(selected_job.into()),
					store,
				}
				.update()?;
			}};
		}

		match store.adapter
		{
			#[cfg(feature = "bincode")]
			Adapters::Bincode => retrieve!(
				BincodeEmployee,
				BincodeJob,
				BincodeLocation,
				BincodeOrganization,
				BincodePerson
			),

			_ => return Err(AdapterError::FeatureNotFound(store.adapter).into()),
		};

		Ok(())
	}
}
