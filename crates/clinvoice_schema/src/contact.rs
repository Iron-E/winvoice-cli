mod contact_kind;
mod display;
mod restorable_serde;

pub use contact_kind::ContactKind;
#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

/// A way that the [`Organization`](super::Organization) which uses CLInvoice can be contacted by
/// another [`Organization`]/entity which has been given an [`Invoice`](super::Invoice)/exported
/// [`Job`](super::Job) in order to facilitate payment or make a request for further services.
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Contact
{
	/// See [`ContactKind`].
	pub kind: ContactKind,

	/// The reference label of this [`Contact`], which is human-readable and easily
	/// contextualizes the information in `kind`.
	///
	/// Should be assigned by a user, and updatable as well.
	pub label: String,
}
