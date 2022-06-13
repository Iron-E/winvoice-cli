mod contact_kind;
mod display;
mod restorable_serde;

pub use contact_kind::ContactKind;
#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

/// # Summary
///
/// A piece of [`Contact`] information for an [`Employee`].
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
pub struct Contact
{
	/// # Summary
	///
	/// The specific information contained by this [`Contact`].
	pub kind: ContactKind,

	/// # Summary
	///
	/// The label for this [`Contact`]. Note that it must be unique per `employee_id`.
	pub label: String,
}
