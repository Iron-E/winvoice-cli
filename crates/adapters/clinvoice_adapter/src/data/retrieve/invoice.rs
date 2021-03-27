use
{
	super::InvoiceDate,
	crate::data::MatchWhen,
	clinvoice_data::Money,
};

#[cfg(feature="serde_support")]
use serde::{Deserialize, Serialize};

/// # Summary
///
/// An [`Invoice`](clinvoice_data::Invoice) with [matchable](MatchWhen) fields.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature="serde_support", derive(Deserialize, Serialize))]
pub struct Invoice<'m>
{
	#[cfg_attr(feature="serde_support", serde(default))]
	pub date: Option<InvoiceDate<'m>>,

	#[cfg_attr(feature="serde_support", serde(default))]
	pub hourly_rate: MatchWhen<'m, Money>,
}

impl Invoice<'_>
{
	/// # Summary
	///
	/// Return `true` if `invoice` is a match.
	pub fn matches(&self, invoice: &clinvoice_data::Invoice) -> bool
	{
		todo!()
	}
}
