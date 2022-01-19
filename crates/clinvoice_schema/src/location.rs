mod display;
mod restorable_serde;

#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

use crate::Id;

/// # Summary
///
/// A view of [`Location`](crate::Location).
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
pub struct Location
{
	/// # Summary
	///
	/// The reference number of the [`Location`].
	#[cfg_attr(feature = "serde_support", serde(skip))]
	pub id: Id,

	/// # Summary
	///
	/// The [`Location`][location]s which this [`Location`][location] is inside of.
	///
	/// * In order of innermost to outermost.
	///
	/// [location]: crate::Location
	pub outer: Option<Box<Self>>,

	/// # Summary
	///
	/// The name of the [`Location`].
	pub name: String,
}

impl Location
{
	/// # Summary
	///
	/// Create a new [`Location`].
	pub fn new(id: Id, name: String, outer: Option<&Self>) -> Self
	{
		Self {
			id,
			name,
			outer: outer.map(|l| l.clone().into()),
		}
	}
}
