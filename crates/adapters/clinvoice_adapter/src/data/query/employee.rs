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
/// An [`Employee`](clinvoice_data::Employee) with [matchable](Match) fields.
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
	pub fn set_matches_view<'item>(&self, mut employees: impl Iterator<Item=&'item EmployeeView>) -> bool
	{
		self.contact_info.set_matches_view(employees.by_ref().map(|e| e.contact_info.values()).flatten()) &&
		self.id.set_matches(&employees.by_ref().map(|e| &e.id).collect()) &&
		self.organization.set_matches_view(employees.by_ref().map(|e| &e.organization)) &&
		self.person.set_matches_view(employees.by_ref().map(|e| &e.person)) &&
		self.title.set_matches(&employees.by_ref().map(|e| &e.title).collect()) &&
		self.status.set_matches(&employees.map(|e| &e.status).collect())
	}
}
