use
{
	super::InvoiceDate,
	crate::data::Match,
	clinvoice_data::Money,
};

#[cfg(feature="serde_support")]
use serde::{Deserialize, Serialize};

/// # Summary
///
/// An [`Invoice`](clinvoice_data::Invoice) with [matchable](MatchWhen) fields.
#[derive(Clone, Default, Debug, Eq, PartialEq)]
#[cfg_attr(feature="serde_support", derive(Deserialize, Serialize))]
pub struct Invoice<'m>
{
	#[cfg_attr(feature="serde_support", serde(default))]
	pub date: Option<InvoiceDate<'m>>,

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
		match &self.date {
			Some(date) => invoice.date.as_ref().map(|d| date.matches(d)).unwrap_or(false),
			_ => invoice.date.is_none(),
		}
	}
}
