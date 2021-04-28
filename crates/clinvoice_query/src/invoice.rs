use
{
	super::Match,

	clinvoice_data::{chrono::NaiveDateTime, Money},
};

#[cfg(feature="serde_support")]
use serde::{Deserialize, Serialize};

/// # Summary
///
/// An [`Invoice`](clinvoice_data::Invoice) with [matchable](Match) fields.
#[derive(Clone, Default, Debug, Eq, PartialEq)]
#[cfg_attr(feature="serde_support", derive(Deserialize, Serialize))]
pub struct Invoice<'m>
{
	#[cfg_attr(feature="serde_support", serde(default))]
	pub issued: Match<'m, Option<NaiveDateTime>>,

	#[cfg_attr(feature="serde_support", serde(default))]
	pub paid: Match<'m, Option<NaiveDateTime>>,

	#[cfg_attr(feature="serde_support", serde(default))]
	pub hourly_rate: Match<'m, Money>,
}

impl Invoice<'_>
{
	/// # Summary
	///
	/// Return `true` if `invoice` is a match.
	pub fn matches(&self, invoice: &clinvoice_data::Invoice) -> bool
	{
		self.hourly_rate.matches(&invoice.hourly_rate) &&
		self.issued.matches(&invoice.date.as_ref().map(|d| d.issued.naive_local())) &&
		self.paid.matches(&invoice.date.as_ref().and_then(|d| d.paid.map(|p| p.naive_local())))
	}
}
