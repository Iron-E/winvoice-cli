use
{
	crate::data::MatchWhen,
	clinvoice_data::Id,
};

/// # Summary
///
/// An [`Location`](clinvoice_data::Location) with [matchable](MatchWhen) fields.
pub struct Location<'m>
{
	pub id: MatchWhen<'m, Id>,
	pub outer: Option<Box<Self>>,
	pub name: MatchWhen<'m, String>,
}
