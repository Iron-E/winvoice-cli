use clinvoice_finance::ExchangeRates;
use clinvoice_schema::{Contact, Job, Organization, Timesheet};

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Format
{
	#[cfg(feature = "markdown")]
	Markdown,
}

impl Format
{
	/// # Summary
	///
	/// Export some `job` to the [`Target`] specified. `contact_info` and `timesheets` are exported
	/// in the order given.
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
			_ => String::new(),
		}
	}
}
