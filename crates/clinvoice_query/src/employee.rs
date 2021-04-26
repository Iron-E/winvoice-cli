use
{
	super::{Contact, Match, MatchStr, Organization, Person, Result},

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
	pub title: MatchStr<String>,

	#[cfg_attr(feature="serde_support", serde(default))]
	pub status: Match<'m, EmployeeStatus>,
}

impl Employee<'_>
{
	/// # Summary
	///
	/// Return `true` if `employee` is a match.
	pub fn matches(&self, employee: &clinvoice_data::Employee) -> Result<bool>
	{
		Ok(
			self.contact_info.set_matches(&mut employee.contact_info.values())? &&
			self.id.matches(&employee.id) &&
			self.organization.id.matches(&employee.organization_id) &&
			self.person.id.matches(&employee.person_id) &&
			self.title.matches(&employee.title)? &&
			self.status.matches(&employee.status)
		)
	}

	/// # Summary
	///
	/// Return `true` if `employee` is a match.
	pub fn matches_view(&self, employee: &EmployeeView) -> Result<bool>
	{
		Ok(
			self.contact_info.set_matches_view(&mut employee.contact_info.values())? &&
			self.id.matches(&employee.id) &&
			self.organization.matches_view(&employee.organization)? &&
			self.person.matches_view(&employee.person)? &&
			self.title.matches(&employee.title)? &&
			self.status.matches(&employee.status)
		)
	}

	/// # Summary
	///
	/// Return `true` if `employee` is a match.
	pub fn set_matches_view<'item>(&self, employees: &mut impl Iterator<Item=&'item EmployeeView>) -> Result<bool>
	{
		Ok(
			self.contact_info.set_matches_view(&mut employees.by_ref().map(|e| e.contact_info.values()).flatten())? &&
			self.id.set_matches(&employees.by_ref().map(|e| &e.id).collect()) &&
			self.organization.set_matches_view(&mut employees.by_ref().map(|e| &e.organization))? &&
			self.person.set_matches_view(&mut employees.by_ref().map(|e| &e.person))? &&
			self.title.set_matches(&mut employees.by_ref().map(|e| e.title.as_ref()))? &&
			self.status.set_matches(&employees.map(|e| &e.status).collect())
		)
	}
}
