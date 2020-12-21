use crate::Id;

/// # Summary
///
/// A physical space where other `Location`s or
/// [`Organization`](super::organization::Organization)s exist.
pub struct Location<'name>
{
	/// # Summary
	///
	/// The reference number of the [`Location`].
	pub id: Id,

	/// # Summary
	///
	/// The reference number of the [`Location`] in which _this_ [`Location`] resides.
	pub outer_id: Option<Id>,

	/// # Summary
	///
	/// The name of the [`Location`].
	pub name: &'name str,
}
