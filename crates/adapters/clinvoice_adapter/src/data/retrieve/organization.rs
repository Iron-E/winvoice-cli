use
{
	super::Location,
	crate::data::MatchWhen,
	clinvoice_data::Id,
};

/// # Summary
///
/// An [`Organization`](clinvoice_data::Organization) with [matchable](MatchWhen) fields.
pub struct Organization<'m>
{
	pub id: MatchWhen<'m, Id>,
	pub location: Location<'m>,
	pub name: MatchWhen<'m, String>,
}
