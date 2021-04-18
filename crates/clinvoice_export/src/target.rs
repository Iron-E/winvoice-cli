mod error;
mod from_str;

pub use error::{Error, Result};

use
{
	core::fmt::Write,

	crate::markdown,

	clinvoice_data::
	{
		chrono::{DateTime, Local},
		Job,
		views::{JobView, TimesheetView},
	},
};

/// # Summary
///
/// A target for exporting.
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Target
{
	/// # Summary
	///
	/// The markdown target. Exports to a `.md` file.
	#[cfg(feature="markdown")]
	Markdown,
}

impl Target
{
	/// # Summary
	///
	/// Export some `job` to the [`Target`] specified.
	fn export_timesheet(&self, output: &mut String, timesheet: &TimesheetView)
	{
		match self
		{
			#[cfg(feature="markdown")]
			Self::Markdown =>
			{

				writeln!(output, "{}", markdown::Element::Heading
				{
					depth: 3,
					text: format!("{} – {}",
						DateTime::<Local>::from(timesheet.time_begin),
						timesheet.time_end.map(|time| DateTime::<Local>::from(time).to_string()).unwrap_or_else(|| "Current".into())
					),
				}).unwrap();

				writeln!(output, "{}", markdown::Element::Heading {depth: 4, text: "Employee Information"}).unwrap();
				writeln!(output, "{}: {}",
					markdown::Element::UnorderedList {depth: 0, text: markdown::Text::Bold("Employer")},
					timesheet.employee.organization,
				).unwrap();

				if !timesheet.employee.contact_info.is_empty()
				{
					writeln!(output, "{}:", markdown::Element::UnorderedList {depth: 0, text: markdown::Text::Bold("Contact Information")}).unwrap();

					let mut sorted_employee_contact_info: Vec<&String> = timesheet.employee.contact_info.keys().collect();
					sorted_employee_contact_info.sort();
					sorted_employee_contact_info.into_iter().try_for_each(|label| writeln!(output, "{}: {}",
						markdown::Element::UnorderedList {depth: 1, text: markdown::Text::Bold(label)},
						timesheet.employee.contact_info[label],
					)).unwrap();
				}

				if !timesheet.expenses.is_empty()
				{
					writeln!(output, "{}", markdown::Element::Heading {depth: 4, text: "Expenses"}).unwrap();

					timesheet.expenses.iter().try_for_each(|e| writeln!(output, "{}{}",
						markdown::Element::Heading {depth: 5, text: format!("{} – {}", e.category, e.cost)},
						markdown::Element::BlockText(&e.description),
					)).unwrap();
				}

				if !timesheet.work_notes.is_empty()
				{
					writeln!(output, "{}", markdown::Element::Heading {depth: 4, text: "Work Notes"}).unwrap();
					writeln!(output, "{}", markdown::Element::BlockText(&timesheet.work_notes)).unwrap();
				}
			},
		};
	}

	/// # Summary
	///
	/// Export some `job` to the [`Target`] specified.
	pub fn export_job(&self, job: JobView) -> String
	{
		let mut output = String::new();

		match self
		{
			#[cfg(feature="markdown")]
			Self::Markdown =>
			{
				writeln!(output, "{}", markdown::Element::Heading {depth: 1, text: format!("Job #{} for {}", job.id, job.client)}).unwrap();

				writeln!(output, "{}: {}",
					markdown::Element::UnorderedList {depth: 0, text: markdown::Text::Bold("Date Opened")},
					DateTime::<Local>::from(job.date_open),
				).unwrap();

				if let Some(date) = job.date_close
				{
					writeln!(output, "{}: {}",
						markdown::Element::UnorderedList {depth: 0, text: markdown::Text::Bold("Date Closed")},
						DateTime::<Local>::from(date),
					).unwrap();
				}

				writeln!(output, "{}", markdown::Element::<&str>::Break).unwrap();

				writeln!(output, "{}", markdown::Element::Heading {depth: 2, text: "Invoice"}).unwrap();
				writeln!(output, "{} {}",
					markdown::Element::UnorderedList {depth: 0, text: markdown::Text::Bold("Hourly Rate")},
					job.invoice.hourly_rate,
				).unwrap();

				if let Some(date) = &job.invoice.date
				{
					writeln!(output, "{}: {}",
						markdown::Element::UnorderedList {depth: 0, text: markdown::Text::Bold("Status")},
						date,
					).unwrap();
				}

				writeln!(output, "{}: {}",
					markdown::Element::UnorderedList {depth: 0, text: markdown::Text::Bold("Total Amount Owed")},
					Job::from(&job).total(),
				).unwrap();
				writeln!(output, "{}", markdown::Element::<&str>::Break).unwrap();

				writeln!(output, "{}", markdown::Element::Heading {depth: 2, text: "Objectives"}).unwrap();
				writeln!(output, "{}", markdown::Element::BlockText(&job.objectives)).unwrap();

				if !job.notes.is_empty()
				{
					writeln!(output, "{}", markdown::Element::Heading {depth: 2, text: "Notes"}).unwrap();
					writeln!(output, "{}", markdown::Element::BlockText(&job.notes)).unwrap();
				}

				if !job.timesheets.is_empty()
				{
					writeln!(output, "{}", markdown::Element::Heading {depth: 2, text: "Timesheets"}).unwrap();
					job.timesheets.iter().for_each(|t| self.export_timesheet(&mut output, t));
				}
			},
		};

		output
	}
}
