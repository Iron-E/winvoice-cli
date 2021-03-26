use
{
	crate::data::MatchWhen,
	clinvoice_data::chrono::{DateTime, Utc},
};

/// # Summary
///
/// An [`InvoiceDate`](clinvoice_data::InvoiceDate) with [matchable](MatchWhen) fields.
pub struct InvoiceDate<'m>
{
	pub issued: MatchWhen<'m, DateTime<Utc>>,
	pub paid: MatchWhen<'m, Option<DateTime<Utc>>>,
}
