use
{
	crate::data::MatchWhen,
	clinvoice_data::Id,
};

/// # Summary
///
/// An [`Location`](clinvoice_data::Location) with [matchable](MatchWhen) fields.
pub struct Person<'m>
{
	pub id: MatchWhen<'m, Id>,
	pub name: MatchWhen<'m, String>,
}
