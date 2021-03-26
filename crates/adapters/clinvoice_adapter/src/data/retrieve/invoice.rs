use
{
	super::InvoiceDate,
	crate::data::MatchWhen,
	clinvoice_data::Money,
};

/// # Summary
///
/// An [`Invoice`](clinvoice_data::Invoice) with [matchable](MatchWhen) fields.
pub struct Invoice<'m>
{
	pub date: Option<InvoiceDate<'m>>,
	pub hourly_rate: MatchWhen<'m, Money>,
}
