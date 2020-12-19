use super::id::Id;

/// TODO
pub struct Person<'name>
{
	/// TODO
	_id: Id,
	/// TODO
	pub name: &'name str,
}
