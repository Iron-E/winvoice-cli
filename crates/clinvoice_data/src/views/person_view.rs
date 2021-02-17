mod hash;

use
{
	super::ContactView,
	crate::Id,
	std::collections::HashSet,
};

#[cfg(feature="serde_support")]
use serde::{Deserialize, Serialize};

/// # Summary
//
/// A view of [`Person`](crate::Person).
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature="serde_support", derive(Deserialize, Serialize))]
pub struct PersonView
{
	/// # Summary
	///
	/// Contact information specific to the individual [`Person`], rather than a corporation they
	/// work at.
	pub contact_info: HashSet<ContactView>,

	/// # Summary
	///
	/// This is the unique reference number for the [`Person`].
	///
	/// # Remarks
	///
	/// The other `View` structures do not contain an `id` field because they have enough
	/// information for unique identification and hashing. However, the [`Person`] requires this
	/// field in order to be uniquely identified.
	pub id: Id,

	/// # Summary
	///
	/// This is the name of the [`Person`].
	pub name: String,
}
