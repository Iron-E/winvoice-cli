mod display;

use core::fmt::Write;
use std::collections::HashSet;

use chrono::{DateTime, Utc};
#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

use super::{
	markdown::{Element, Text},
	ContactView,
	EmployeeView,
	JobView,
};
use crate::{Expense, Id};

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
	/// The ID of the [`Job`](crate::Job) this [`Timesheet`] is attached to.
	pub job: JobView,

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

impl TimesheetView
{
	/// # Summary
	///
	/// Export some `job` to the [`Target`] specified. Appends to some pre-existing `output`, in
	/// case multiple [`TimesheetView`]s must be serialized sequentially.
	///
	/// Tracks the previously `serialized_employees` so that their contact information is not
	/// reiterated every time.
	pub(super) fn export(&self, serialized_employees: &mut HashSet<Id>, output: &mut String)
	{
		writeln!(output, "{}", Element::Heading {
			depth: 3,
			text: self
				.time_end
				.map(|time_end| format!("{} – {}", self.time_begin, time_end.naive_local()))
				.unwrap_or_else(|| format!("{} – Current", self.time_begin)),
		})
		.unwrap();

		writeln!(output, "{}", Element::Heading {
			depth: 4,
			text: "Employee Information",
		})
		.unwrap();
		writeln!(
			output,
			"{}: {}",
			Element::UnorderedList {
				depth: 0,
				text: Text::Bold("Name"),
			},
			self.employee.person.name,
		)
		.unwrap();
		writeln!(
			output,
			"{}: {}",
			Element::UnorderedList {
				depth: 0,
				text: Text::Bold("Employer"),
			},
			self.employee.organization,
		)
		.unwrap();
		writeln!(
			output,
			"{}: {}",
			Element::UnorderedList {
				depth: 0,
				text: Text::Bold("Title"),
			},
			self.employee.title,
		)
		.unwrap();

		if serialized_employees.contains(&self.employee.id)
		{
			let employee_contact_info: Vec<_> = self
				.employee
				.contact_info
				.iter()
				.filter(|(_, c)| match c
				{
					ContactView::Address {
						location: _,
						export,
					} => *export,
					ContactView::Email { email: _, export } => *export,
					ContactView::Phone { phone: _, export } => *export,
				})
				.collect();

			if !employee_contact_info.is_empty()
			{
				writeln!(output, "{}:", Element::UnorderedList {
					depth: 0,
					text: Text::Bold("Contact Information"),
				})
				.unwrap();

				let mut sorted_employee_contact_info = employee_contact_info;
				sorted_employee_contact_info.sort_by_key(|(label, _)| *label);

				sorted_employee_contact_info
					.into_iter()
					.try_for_each(|(label, contact)| {
						writeln!(output, "{}: {contact}", Element::UnorderedList {
							depth: 1,
							text: Text::Bold(label),
						})
					})
					.unwrap();
			}
		}

		writeln!(output, "{}", Element::<&str>::Break).unwrap();

		if !self.expenses.is_empty()
		{
			writeln!(output, "{}", Element::Heading {
				depth: 4,
				text: "Expenses",
			})
			.unwrap();

			self
				.expenses
				.iter()
				.try_for_each(|e| {
					writeln!(
						output,
						"{}\n{}",
						Element::Heading {
							depth: 5,
							text: format!("{} – {}", e.category, e.cost),
						},
						Element::BlockText(&e.description),
					)
				})
				.unwrap();
		}

		if !self.work_notes.is_empty()
		{
			writeln!(output, "{}", Element::Heading {
				depth: 4,
				text: "Work Notes",
			})
			.unwrap();
			writeln!(output, "{}", Element::BlockText(&self.work_notes)).unwrap();
		}

		serialized_employees.insert(self.employee.id);
	}
}
