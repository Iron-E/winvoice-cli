use
{
	crate::data::Match,
	clinvoice_data::chrono::{DateTime, Utc},
};

#[cfg(feature="serde_support")]
use serde::{Deserialize, Serialize};

/// # Summary
///
/// An [`InvoiceDate`](clinvoice_data::InvoiceDate) with [matchable](MatchWhen) fields.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature="serde_support", derive(Deserialize, Serialize))]
pub struct InvoiceDate<'m>
{
	#[cfg_attr(feature="serde_support", serde(default))]
	pub issued: Match<'m, DateTime<Utc>>,

	#[cfg_attr(feature="serde_support", serde(default))]
	pub paid: Match<'m, Option<DateTime<Utc>>>,
}

impl InvoiceDate<'_>
{
	/// # Summary
	///
	/// Return `true` if `invoice_date` is a match.
	pub fn matches(&self, invoice_date: &clinvoice_data::InvoiceDate) -> bool
	{
		self.issued.matches(&invoice_date.issued) &&
		self.paid.matches(&invoice_date.paid)
	}
}
