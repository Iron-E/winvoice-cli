use
{
	super::{Match, MatchStr, Result},

	clinvoice_data::{Id, views::PersonView},
};

#[cfg(feature="serde_support")]
use serde::{Deserialize, Serialize};

/// # Summary
///
/// An [`Location`](clinvoice_data::Location) with [matchable](Match) fields.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature="serde_support", derive(Deserialize, Serialize))]
pub struct Person<'m>
{
	#[cfg_attr(feature="serde_support", serde(default))]
	pub id: Match<'m, Id>,

	#[cfg_attr(feature="serde_support", serde(default))]
	pub name: MatchStr<String>,
}

impl Person<'_>
{
	/// # Summary
	///
	/// Return `true` if `person` is a match.
	pub fn matches(&self, person: &clinvoice_data::Person) -> Result<bool>
	{
		Ok(
			self.id.matches(&person.id) &&
			self.name.matches(&person.name)?
		)
	}

	/// # Summary
	///
	/// Return `true` if `person` is a match.
	pub fn matches_view(&self, person: &PersonView) -> Result<bool>
	{
		Ok(
			self.id.matches(&person.id) &&
			self.name.matches(&person.name)?
		)
	}

	/// # Summary
	///
	/// Return `true` if `people` [`Match::set_matches`].
	pub fn set_matches_view<'item>(&self, mut people: impl Iterator<Item=&'item PersonView>) -> Result<bool>
	{
		Ok(
			self.id.set_matches(&people.by_ref().map(|p| &p.id).collect()) &&
			self.name.set_matches(people.map(|p| p.name.as_ref()))?
		)
	}
}
