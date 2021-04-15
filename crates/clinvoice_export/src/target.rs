mod error;
mod from_str;

pub use error::{Error, Result};

use
{
	crate::markdown,

	clinvoice_data::
	{
		chrono::{DateTime, Local},
		views::JobView,
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
	pub fn export_job(&self, job: JobView) -> String
	{
		match self
		{
			#[cfg(feature="markdown")]
			Self::Markdown =>
			[
				markdown::Element::Heading {depth: 1, text: &format!("Job #{} for {}", job.id, job.client)}.render(),
				markdown::Element::BlockText(&format!(
					"{} {}\n{} {}",
					markdown::Text::Bold("Date Opened:").render(),
					DateTime::<Local>::from(job.date_open).to_string(),
					markdown::Text::Bold("Date Closed:").render(),
					job.date_close.map(|date| DateTime::<Local>::from(date).to_string()).unwrap_or_else(|| "Current".into()),
				)).render(),
				markdown::Element::Heading {depth: 2, text: "Invoice"}.render(),
				// TODO: fill out invoice (w/ total amount owed)
				markdown::Element::Heading {depth: 2, text: "Objectives"}.render(),
				// TODO: fill out objectives as a `BlockText`
				markdown::Element::Heading {depth: 2, text: "Notes"}.render(),
				// TODO: fill out notes as a `BlockText`
				markdown::Element::Heading {depth: 2, text: "Timesheets"}.render(),
				// TODO: fill out timesheets
			].join(""),
		}
	}
}
