use
{
	super::{Contact, Organization, Person},
	crate::data::Match,
	clinvoice_data::{EmployeeStatus, Id, views::EmployeeView},
};

#[cfg(feature="serde_support")]
use serde::{Deserialize, Serialize};

/// # Summary
///
/// An [`Employee`](clinvoice_data::Employee) with [matchable](MatchWhen) fields.
#[derive(Clone, Default, Debug, Eq, PartialEq)]
#[cfg_attr(feature="serde_support", derive(Deserialize, Serialize))]
pub struct Employee<'m>
{
	#[cfg_attr(feature="serde_support", serde(default))]
	pub contact_info: Contact<'m>,

	#[cfg_attr(feature="serde_support", serde(default))]
	pub id: Match<'m, Id>,

	#[cfg_attr(feature="serde_support", serde(default))]
	pub organization: Organization<'m>,

	#[cfg_attr(feature="serde_support", serde(default))]
	pub person: Person<'m>,

	#[cfg_attr(feature="serde_support", serde(default))]
	pub title: Match<'m, String>,

	#[cfg_attr(feature="serde_support", serde(default))]
	pub status: Match<'m, EmployeeStatus>,
}

impl Employee<'_>
{
	/// # Summary
	///
	/// Return `true` if `employee` is a match.
	pub fn matches(&self, employee: &clinvoice_data::Employee) -> bool
	{
		self.contact_info.set_matches(employee.contact_info.values()) &&
		self.id.matches(&employee.id) &&
		self.organization.id.matches(&employee.organization_id) &&
		self.person.id.matches(&employee.person_id) &&
		self.title.matches(&employee.title) &&
		self.status.matches(&employee.status)
	}

	/// # Summary
	///
	/// Return `true` if `employee` is a match.
	pub fn matches_view(&self, employee: &EmployeeView) -> bool
	{
		self.contact_info.any_matches_view(employee.contact_info.values()) &&
		self.id.matches(&employee.id) &&
		self.organization.matches_view(&employee.organization) &&
		self.person.matches_view(&employee.person) &&
		self.title.matches(&employee.title) &&
		self.status.matches(&employee.status)
	}
}
