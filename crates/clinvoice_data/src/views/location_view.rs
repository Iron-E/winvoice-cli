mod display;

use crate::Id;

#[cfg(feature="serde_support")]
use serde::{Deserialize, Serialize};

/// # Summary
///
/// A view of [`Location`](crate::Location).
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature="serde_support", derive(Deserialize, Serialize))]
pub struct LocationView
{
	/// # Summary
	///
	/// The reference number of the [`Location`].
	#[cfg_attr(feature="serde_support", serde(skip_serializing))]
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

impl LocationView
{
	/// # Summary
	///
	/// Create a new [`LocationView`].
	pub fn new(id: Id, name: String, outer: Option<&Self>) -> Self
	{
		Self
		{
			id,
			name,
			outer: outer.map(|l| l.clone().into()),
		}
	}
}
