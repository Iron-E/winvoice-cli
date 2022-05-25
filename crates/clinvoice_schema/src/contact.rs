mod contact_kind;
mod display;
mod restorable_serde;

pub use contact_kind::ContactKind;
#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

use crate::Id;

/// # Summary
///
/// A piece of [`Contact`] information for an [`Employee`].
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
pub struct Contact
{
	/// # Summary
	///
	/// Whether to show this piece of contact information on exported [`Job`]s.
	pub export: bool,

	/// # Summary
	///
	/// The specific information contained by this [`Contact`].
	pub kind: ContactKind,

	/// # Summary
	///
	/// The label for this [`Contact`]. Note that it must be unique per `employee_id`.
	pub label: String,

	/// # Summary
	///
	/// The [`Id`] of the [`Employee`] who this [`Contact`] belongs to.
	#[cfg_attr(feature = "serde_support", serde(skip))]
	pub organization_id: Id,
}
