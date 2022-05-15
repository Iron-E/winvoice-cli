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
	#[cfg_attr(feature = "serde_support", serde(skip))]
	pub employee_id: Id,
	pub export: bool,
	pub kind: ContactKind,
	pub label: String,
}
