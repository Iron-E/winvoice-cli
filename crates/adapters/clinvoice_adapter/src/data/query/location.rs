mod outer_location;

use
{
	crate::data::Match,

	clinvoice_data::{Id, views::LocationView},
};

#[cfg(feature="serde_support")]
use serde::{Deserialize, Serialize};

pub use outer_location::OuterLocation;

/// # Summary
///
/// An [`Location`](clinvoice_data::Location) with [matchable](Match) fields.
#[derive(Clone, Default, Debug, Eq, PartialEq)]
#[cfg_attr(feature="serde_support", derive(Deserialize, Serialize))]
pub struct Location<'m>
{
	#[cfg_attr(feature="serde_support", serde(default))]
	pub id: Match<'m, Id>,

	#[cfg_attr(feature="serde_support", serde(default))]
	pub outer: OuterLocation<'m>,

	#[cfg_attr(feature="serde_support", serde(default))]
	pub name: Match<'m, String>,
}

impl Location<'_>
{
	/// # Summary
	///
	/// Return `true` if `location` is a match.
	pub fn matches(&self, location: &clinvoice_data::Location) -> bool
	{
		self.id.matches(&location.id) &&
		match &self.outer
		{
			OuterLocation::Some(outer) => location.outer_id.as_ref().map(|id| outer.id.matches(&id)).unwrap_or(false),
			OuterLocation::None => location.outer_id.is_none(),
			_ => true,
		} &&
		self.name.matches(&location.name)
	}

	/// # Summary
	///
	/// Return `true` if `location` is a match.
	pub fn matches_view(&self, location: &LocationView) -> bool
	{
		self.id.matches(&location.id) &&
		match &self.outer
		{
			OuterLocation::Some(outer) => location.outer.as_ref().map(|o| outer.matches_view(&o)).unwrap_or(false),
			OuterLocation::None => location.outer.is_none(),
			_ => true,
		} &&
		self.name.matches(&location.name)
	}
}
