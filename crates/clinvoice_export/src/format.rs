use clinvoice_finance::ExchangeRates;
use clinvoice_schema::{Contact, Job, Organization, Timesheet};

/// A [file format](https://en.wikipedia.org/wiki/File_format) to export information to.
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Format
{
	/// The [markdown](https://en.wikipedia.org/wiki/Markdown) file format.
	#[cfg(feature = "markdown")]
	Markdown,
}

impl Format
{
	/// Export some `job` to [Markdown](crate::Format::Markdown).
	///
	/// `contact_info` and `timesheets` are exported in the order given.
	///
	/// # Warnings
	///
	/// * The following fields must all contain valid markdown syntax:
	///   * The `objectives` and `notes` of the `job`.
	///   * The `work_notes` of every [`Timesheet`] of the `timesheets`.
	///   * The `category` and `description` of every [`Expense`] of the `expenses` of every
	///     [`Timesheet`] of the `timesheets`.
	///
	/// # Panics
	///
	/// * When [`Timesheet::total`](clinvoice_schema::Timesheet::total) does.
	pub fn export_job(
		&self,
		job: &Job,
		contact_info: &[Contact],
		exchange_rates: Option<&ExchangeRates>,
		organization: &Organization,
		timesheets: &[Timesheet],
	) -> String
	{
		match self
		{
			#[cfg(feature = "markdown")]
			Self::Markdown =>
			{
				crate::markdown::export_job(job, contact_info, exchange_rates, organization, timesheets)
			},

			// NOTE: this is allowed because there may be additional formats added later, and I want
			//       to define this behavior now.
			#[allow(unreachable_patterns)]
			_ => panic!("`clinvoice_export` was not compiled to support any file formats."),
		}
	}
}
