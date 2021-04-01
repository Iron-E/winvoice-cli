use
{
	crate::data::Match,
	clinvoice_data::{Id, views::PersonView},
};

#[cfg(feature="serde_support")]
use serde::{Deserialize, Serialize};

/// # Summary
///
/// An [`Location`](clinvoice_data::Location) with [matchable](MatchWhen) fields.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature="serde_support", derive(Deserialize, Serialize))]
pub struct Person<'m>
{
	#[cfg_attr(feature="serde_support", serde(default))]
	pub id: Match<'m, Id>,

	#[cfg_attr(feature="serde_support", serde(default))]
	pub name: Match<'m, String>,
}

impl Person<'_>
{
	/// # Summary
	///
	/// Return `true` if `person` is a match.
	pub fn matches(&self, person: &clinvoice_data::Person) -> bool
	{
		self.id.matches(&person.id) &&
		self.name.matches(&person.name)
	}

	/// # Summary
	///
	/// Return `true` if `person` is a match.
	pub fn matches_view(&self, person: &PersonView) -> bool
	{
		self.id.matches(&person.id) &&
		self.name.matches(&person.name)
	}
}
