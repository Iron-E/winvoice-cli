mod default;

use
{
	crate::data::MatchWhen,
	clinvoice_data::{Id, views::LocationView},
};

#[cfg(feature="serde_support")]
use serde::{Deserialize, Serialize};

/// # Summary
///
/// An [`Location`](clinvoice_data::Location) with [matchable](MatchWhen) fields.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature="serde_support", derive(Deserialize, Serialize))]
pub struct Location<'m>
{
	#[cfg_attr(feature="serde_support", serde(default))]
	pub id: MatchWhen<'m, Id>,

	#[cfg_attr(feature="serde_support", serde(default = "location_outer_default"))]
	pub outer: Result<Box<Self>, bool>,

	#[cfg_attr(feature="serde_support", serde(default))]
	pub name: MatchWhen<'m, String>,
}

const fn location_outer_default<'m>() -> Result<Box<Location<'m>>, bool>
{
	Err(true)
}

impl Location<'_>
{
	/// # Summary
	///
	/// Return `true` if `location` is a match.
	pub fn any_matches_view(&self, locations: &[&LocationView]) -> bool
	{
		locations.iter().any(|l| self.matches_view(l))
	}

	/// # Summary
	///
	/// Return `true` if `location` is a match.
	pub fn matches(&self, location: &clinvoice_data::Location) -> bool
	{
		self.id.matches(&location.id) &&
		match &self.outer
		{
			Ok(outer) => location.outer_id.as_ref().map(|id| outer.id.matches(&id)).unwrap_or(false),
			Err(exists) => location.outer_id.is_some() == *exists,
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
			Ok(outer) => location.outer.as_ref().map(|o| outer.matches_view(&o)).unwrap_or(false),
			Err(exists) => location.outer.is_some() == *exists,
		} &&
		self.name.matches(&location.name)
	}
}
