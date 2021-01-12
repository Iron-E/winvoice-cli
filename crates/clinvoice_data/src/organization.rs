use crate::Id;

use std::collections::HashSet;

/// # Summary
///
/// An `Organization` is a facilitator of business.
///
/// # Remarks
///
/// An `Organization` can be a person, or an entire business. If one is self-employed, then the
/// `Organization` would simply be themselves.
///
/// An `Organization` has no specific affitilation to the user, and as such can be both a
/// Client and an Emlpoyer at the same time.
pub struct Organization<'name>
{
	/// # Summary
	///
	/// The unique reference number for this [`Organization`].
	pub id: Id,

	/// # Summary
	///
	/// The reference umber of the [`Location`](super::location::Location) where this
	/// [`Organization`] resides.
	pub location_id: Id,

	/// # Summary
	///
	/// The name of the [`Organization`].
	pub name: &'name str,

	/// # Summary
	///
	/// The [`Employee`](crate::Employee)s who represent the [`Organization`].
	///
	/// # Example
	///
	/// ```ignore
	/// [representatives]
	/// chief_executive_officer = 1
	/// chief_operations_officer = 2
	/// director_of_finance = 43
	/// ```
	pub representatives: HashSet<Id>,
}
