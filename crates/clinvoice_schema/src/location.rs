mod display;
mod restorable_serde;

#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

use super::Id;

/// A place in the real world where other parts of the schema can reside.
///
/// # Example
///
/// ```rust
/// use clinvoice_schema::Location;
///
/// let _ = Location {
///   id: 0, // NOTE: you normally want to avoid assigning an arbitrary ID like this
///   name: "New York".into(),
///   outer: Some(Location {
///     id: 0, // NOTE: you normally want to avoid assigning an arbitrary ID like this
///     name: "USA".into(),
///     outer: None,
///   }.into()),
/// };
/// ```
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Location
{
	/// The reference number of this [`Location`], which is unique among all [`Location`]s.
	///
	/// Should be generated by a database, and never altered once assigned.
	#[cfg_attr(feature = "serde_support", serde(skip))]
	pub id: Id,

	/// The name of the [`Location`].
	pub name: String,

	/// The [`Location`] which immediately surrounds this one, such that when `outer` is [`None`],
	/// this [`Location`] must be at the outermost scope which is relevant to the
	/// [`Organization`](super::Organization) using CLInvoice.
	pub outer: Option<Box<Self>>,
}
