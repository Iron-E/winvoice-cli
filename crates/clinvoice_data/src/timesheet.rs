use crate::Id;

use chrono::{DateTime, TimeZone};

/// # Summary
///
/// A `Timesheet` contains all information pertaining to work that has been performed during a
/// specific period of time while working on a [`Job`](super::job::Job)
///
/// # Remarks
///
/// It is likely that a given CLInvoice business object will contain multiple timesheets. As such,
/// it is proposed that the container for business logic contain an array of `Timesheet`, rather
/// than only one.
pub struct Timesheet<'work_notes, TZone> where TZone : TimeZone
{
	/// # Summary
	///
	/// The ID of the [`Employee`](crate::Employee) who performed this work.
	pub employee_id: Id,

	/// # Summary
	///
	/// The ID of the [`Job`](crate::Job) which this [`Timesheet`] is in reference to.
	pub job_id: Id,

	/// # Summary
	///
	/// The time at which this period of work began.
	pub time_begin: DateTime<TZone>,

	/// # Summary
	///
	/// The time at which this period of work ended.
	pub time_end: Option<DateTime<TZone>>,

	/// # Summary
	///
	/// A summary of what work was performed
	///
	/// # Example
	///
	/// > __Note:__ the `str` may contain any valid markdown.
	///
	/// ```markdown
	/// * Researched alternative solutions to image rendering issue.
	/// * Implemented chosen solution.
	/// * Created tests for chosen solution.
	/// ```
	pub work_notes: &'work_notes str,
}
