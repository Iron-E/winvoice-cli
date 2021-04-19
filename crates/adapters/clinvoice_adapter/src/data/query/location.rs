mod outer_location;

use
{
	super::{Match, MatchStr},

	clinvoice_data::{Id, views::LocationView},

	regex::Error,
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
	pub name: MatchStr<String>,
}

impl Location<'_>
{
	/// # Summary
	///
	/// Return `true` if `location` is a match.
	pub fn matches(&self, location: &clinvoice_data::Location) -> Result<bool, Error>
	{
		Ok(
			self.id.matches(&location.id) &&
			match &self.outer
			{
				OuterLocation::Some(outer) => location.outer_id.as_ref().map(|id| outer.id.matches(&id)).unwrap_or(false),
				OuterLocation::None => location.outer_id.is_none(),
				_ => true,
			} &&
			self.name.matches(&location.name)?
		)
	}

	/// # Summary
	///
	/// Return `true` if `location` is a match.
	pub fn matches_view(&self, location: &LocationView) -> Result<bool, Error>
	{
		Ok(
			self.id.matches(&location.id) &&
			match &self.outer
			{
				OuterLocation::Some(outer) => location.outer.as_ref().map(|o| outer.matches_view(&o)).unwrap_or(Ok(false))?,
				OuterLocation::None => location.outer.is_none(),
				_ => true,
			} &&
			self.name.matches(&location.name)?
		)
	}

	/// # Summary
	///
	/// Return `true` if `locations` [`Match::set_matches`].
	pub fn set_matches_view<'item>(&self, mut locations: impl Iterator<Item=&'item LocationView>) -> Result<bool, Error>
	{
		Ok(
			self.id.set_matches(&locations.by_ref().map(|l| &l.id).collect()) &&
			match &self.outer
			{
				OuterLocation::Some(outer) => locations.by_ref()
					.filter_map(|l| l.outer.as_ref())
					.try_fold(false, |mut b, o| -> Result<bool, Error>
					{
						if !b { b = outer.matches_view(&o)?; }

						Ok(b)
					})?,
				OuterLocation::None => locations.by_ref().any(|l| l.outer.is_none()),
				_ => true,
			} &&
			self.name.set_matches(locations.map(|l| l.name.as_ref()))?
		)
	}
}
