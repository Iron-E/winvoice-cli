use super::id::Id;

/// TODO
pub struct Location<'name>
{
	/// TODO
	_id: Id,
	/// TODO
	_outer_id: Option<Id>,
	/// TODO
	pub name: &'name str,
}
