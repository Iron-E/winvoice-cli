mod display;

use chrono::{
	DateTime,
	Utc,
};
#[cfg(feature = "serde_support")]
use serde::{
	Deserialize,
	Serialize,
};

use super::EmployeeView;
use crate::Expense;

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
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
pub struct TimesheetView
{
	/// # Summary
	///
	/// The ID of the [`Employee`](crate::Employee) who performed this work.
	pub employee: EmployeeView,

	/// # Summary
	///
	/// [`Expense`]s which were incurred during this time.
	pub expenses: Vec<Expense>,

	/// # Summary
	///
	/// The time at which this period of work began.
	pub time_begin: DateTime<Utc>,

	/// # Summary
	///
	/// The time at which this period of work ended.
	///
	/// # Remarks
	///
	/// Is [`Option`] because the time that a work period ends is not known upon first creation.
	pub time_end: Option<DateTime<Utc>>,

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
	pub work_notes: String,
}
