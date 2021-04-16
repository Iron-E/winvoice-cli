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
	pub fn export_timesheet(&self, output: &mut String, timesheet: &TimesheetView)
	{
		match self
		{
			#[cfg(feature="markdown")]
			Self::Markdown =>
			{

				writeln!(output, "{}", markdown::Element::Heading
				{
					depth: 3,
					text: format!("{} – {}", timesheet.time_begin, timesheet.time_end.expect("Timesheets should be completed before exporting.")),
				}).unwrap();

				writeln!(output, "{}", markdown::Element::BlockText(format!("{} {}",
					markdown::Text::Bold("Employee:"),
					timesheet.employee,
				))).unwrap();

				if !timesheet.expenses.is_empty()
				{
					writeln!(output, "{}", markdown::Element::Heading {depth: 4, text: "Expenses"}).unwrap();

					timesheet.expenses.iter().try_for_each(|e| writeln!(output, "{}{}",
						markdown::Element::Heading {depth: 5, text: format!("{} – {}", e.category, e.cost)},
						markdown::Element::BlockText(e.description.as_str()),
					)).unwrap();
				}

				writeln!(output, "{}", markdown::Element::Heading {depth: 4, text: "Work Notes"}).unwrap();
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

				writeln!(output, "{}", markdown::Element::BlockText(format!("{} {}\n{} {}",
					markdown::Text::Bold("Date Opened:"),
					DateTime::<Local>::from(job.date_open),
					markdown::Text::Bold("Date Closed:"),
					job.date_close.expect("`Job`s should be closed before exporting"),
				))).unwrap();

				writeln!(output, "{}", markdown::Element::Heading {depth: 2, text: "Invoice"}).unwrap();
				writeln!(output, "{}", markdown::Element::BlockText(format!("{} {}\n{} {}\n{} {}",
					markdown::Text::Bold("Hourly Rate:"),
					job.invoice.hourly_rate,
					markdown::Text::Bold("Status:"),
					job.invoice.date.as_ref().expect("Invoice should have issue date before exporting"),
					markdown::Text::Bold("Total Amount Owed:"),
					Job::from(&job).total(),
				))).unwrap();

				writeln!(output, "{}", markdown::Element::Heading {depth: 2, text: "Objectives"}).unwrap();
				writeln!(output, "{}", markdown::Element::BlockText(&job.objectives)).unwrap();

				writeln!(output, "{}", markdown::Element::Heading {depth: 2, text: "Notes"}).unwrap();
				writeln!(output, "{}", markdown::Element::BlockText(&job.notes)).unwrap();

				writeln!(output, "{}", markdown::Element::Heading {depth: 2, text: "Timesheets"}).unwrap();
				job.timesheets.iter().for_each(|t| self.export_timesheet(&mut output, t));
			},
		};

		output
	}
}
