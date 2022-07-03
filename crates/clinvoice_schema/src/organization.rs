mod display;
mod restorable_serde;

#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

use super::{Id, Location};

/// An entity which either requests work (a [`Job`](super::Job) `client`) or performs work for
/// other [`Organization`]s.
///
/// # Notes
///
/// * An `Organization` can be a person (i.e. self-employement), or an entire business.
/// * The [`Organization`] which is using CLInvoice must be in the database. The [`Id`] of this
///   [`Organization`] is configured elsewhere and retrieved as needed.
///
/// # Examples
///
/// ```rust
/// use clinvoice_schema::{Location, Organization};
///
/// let _ = Organization {
///   id: 0, // NOTE: you normally want to avoid assigning an arbitrary ID like this
///   location: Location {
///     id: 0, // NOTE: you normally want to avoid assigning an arbitrary ID like this
///     outer: Some(Location {
///       id: 0, // NOTE: you normally want to avoid assigning an arbitrary ID like this
///       outer: None,
///       name: "Japan".into(),
///     }.into()),
///     name: "Tokyo".into(),
///   },
///   name: "My Company".into(),
/// };
/// ```
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Organization
{
	/// The reference number of this [`Organization`], which is unique among all [`Organization`]s.
	///
	/// Should be generated by a database, and never altered once assigned.
	#[cfg_attr(feature = "serde_support", serde(skip))]
	pub id: Id,

	/// The [`Location`] where this [`Organization`] physically resides.
	pub location: Location,

	/// The name of the [`Organization`].
	pub name: String,
}
