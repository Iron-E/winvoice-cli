use
{
	crate::data::MatchWhen,
	clinvoice_data::chrono::{DateTime, Utc},
	serde::{Deserialize, Serialize},
};

/// # Summary
///
/// An [`InvoiceDate`](clinvoice_data::InvoiceDate) with [matchable](MatchWhen) fields.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature="serde_support", derive(Deserialize, Serialize))]
pub struct InvoiceDate<'m>
{
	#[cfg_attr(feature="serde_support", serde(default))]
	pub issued: MatchWhen<'m, DateTime<Utc>>,

	#[cfg_attr(feature="serde_support", serde(default))]
	pub paid: MatchWhen<'m, Option<DateTime<Utc>>>,
}
