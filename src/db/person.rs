use super::id::Id;

/// # Summary
///
/// A person is a physical human being.
pub struct Person<'name>
{
	/// # Summary
	///
	/// This is the unique reference number for the [`Person`].
	_id: Id,

	/// # Summary
	///
	/// This is the name of the [`Person`].
	pub name: &'name str,
}
