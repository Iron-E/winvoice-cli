use clinvoice_data::{views::OrganizationView, Id};
#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

use super::{Location, Match, MatchStr, Result};

/// # Summary
///
/// An [`Organization`](clinvoice_data::Organization) with [matchable](Match) fields.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
pub struct Organization<'m>
{
	#[cfg_attr(feature = "serde_support", serde(default))]
	pub id: Match<'m, Id>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub location: Location<'m>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub name: MatchStr<String>,
}

impl Organization<'_>
{
	/// # Summary
	///
	/// Return `true` if `organization` is a match.
	pub fn matches(&self, organization: &clinvoice_data::Organization) -> Result<bool>
	{
		Ok(self.id.matches(&organization.id) &&
			self.location.id.matches(&organization.location_id) &&
			self.name.matches(&organization.name)?)
	}

	/// # Summary
	///
	/// Return `true` if `organization` is a match.
	pub fn matches_view(&self, organization: &OrganizationView) -> Result<bool>
	{
		Ok(self.id.matches(&organization.id) &&
			self.location.matches_view(&organization.location)? &&
			self.name.matches(&organization.name)?)
	}

	/// # Summary
	///
	/// Return `true` if `organizations` [`Match::set_matches`].
	pub fn set_matches_view<'item>(
		&self,
		organizations: &mut impl Iterator<Item = &'item OrganizationView>,
	) -> Result<bool>
	{
		Ok(self
			.id
			.set_matches(&organizations.by_ref().map(|o| &o.id).collect()) &&
			self
				.location
				.set_matches_view(&mut organizations.by_ref().map(|o| &o.location))? &&
			self
				.name
				.set_matches(&mut organizations.map(|o| o.name.as_ref()))?)
	}
}
