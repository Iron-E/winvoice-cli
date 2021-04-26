use
{
	super::{Invoice, Match, MatchStr, Organization, Timesheet, Result},

	clinvoice_data::{chrono::NaiveDateTime, Id, views::JobView},
};

#[cfg(feature="serde_support")]
use serde::{Deserialize, Serialize};

/// # Summary
///
/// An [`Job`](clinvoice_data::Job) with [matchable](Match) fields.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature="serde_support", derive(Deserialize, Serialize))]
pub struct Job<'m>
{
	#[cfg_attr(feature="serde_support", serde(default))]
	pub client: Organization<'m>,

	#[cfg_attr(feature="serde_support", serde(default))]
	pub date_close: Match<'m, Option<NaiveDateTime>>,

	#[cfg_attr(feature="serde_support", serde(default))]
	pub date_open: Match<'m, NaiveDateTime>,

	#[cfg_attr(feature="serde_support", serde(default))]
	pub id: Match<'m, Id>,

	#[cfg_attr(feature="serde_support", serde(default))]
	pub invoice: Invoice<'m>,

	#[cfg_attr(feature="serde_support", serde(default))]
	pub notes: MatchStr<String>,

	#[cfg_attr(feature="serde_support", serde(default))]
	pub objectives: MatchStr<String>,

	#[cfg_attr(feature="serde_support", serde(default))]
	pub timesheets: Timesheet<'m>,
}

impl Job<'_>
{
	/// # Summary
	///
	/// Return `true` if `job` is a match.
	pub fn matches(&self, job: &clinvoice_data::Job) -> Result<bool>
	{
		Ok(
			self.client.id.matches(&job.client_id) &&
			self.date_close.matches(&job.date_close.map(|d| d.naive_local())) &&
			self.date_open.matches(&job.date_open.naive_local()) &&
			self.id.matches(&job.id) &&
			self.invoice.matches(&job.invoice) &&
			self.notes.matches(&job.notes)? &&
			self.objectives.matches(&job.objectives)? &&
			self.timesheets.set_matches(&mut job.timesheets.iter())?
		)
	}

	/// # Summary
	///
	/// Return `true` if `job` is a match.
	pub fn matches_view(&self, job: &JobView) -> Result<bool>
	{
		Ok(
			self.client.matches_view(&job.client)? &&
			self.date_close.matches(&job.date_close.map(|d| d.naive_local())) &&
			self.date_open.matches(&job.date_open.naive_local()) &&
			self.id.matches(&job.id) &&
			self.invoice.matches(&job.invoice) &&
			self.notes.matches(&job.notes)? &&
			self.objectives.matches(&job.objectives)? &&
			self.timesheets.set_matches_view(&mut job.timesheets.iter())?
		)
	}
}
