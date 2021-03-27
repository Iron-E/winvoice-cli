mod default;

use
{
	super::{Organization, Person},
	crate::data::MatchWhen,
	clinvoice_data::
	{
		Contact, EmployeeStatus, Id,
		views::{ContactView, EmployeeView},
	},
};

#[cfg(feature="serde_support")]
use serde::{Deserialize, Serialize};

type ContactInfo<'m> = Result<MatchWhen<'m, Contact>, MatchWhen<'m, ContactView>>;

/// # Summary
///
/// An [`Employee`](clinvoice_data::Employee) with [matchable](MatchWhen) fields.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature="serde_support", derive(Deserialize, Serialize))]
pub struct Employee<'m>
{
	#[cfg_attr(feature="serde_support", serde(default="contact_view_default"))]
	pub contact_info: ContactInfo<'m>,

	#[cfg_attr(feature="serde_support", serde(default))]
	pub id: MatchWhen<'m, Id>,

	#[cfg_attr(feature="serde_support", serde(default))]
	pub organization: Organization<'m>,

	#[cfg_attr(feature="serde_support", serde(default))]
	pub person: Person<'m>,

	#[cfg_attr(feature="serde_support", serde(default))]
	pub title: MatchWhen<'m, String>,

	#[cfg_attr(feature="serde_support", serde(default))]
	pub status: MatchWhen<'m, EmployeeStatus>,
}

const fn contact_view_default<'m>() -> ContactInfo<'m>
{
	Err(MatchWhen::Any)
}

impl Employee<'_>
{
	/// # Summary
	///
	/// Return `true` if `employee` is a match.
	pub fn matches(&self, employee: &clinvoice_data::Employee) -> bool
	{
		self.id.matches(&employee.id) &&
		match &self.contact_info
		{
			Ok(match_when) => match_when.set_matches(&employee.contact_info.values().collect()),
			_ => false,
		} &&
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
		self.id.matches(&employee.id) &&
		match &self.contact_info
		{
			Err(match_when) => match_when.set_matches(&employee.contact_info.values().collect()),
			_ => false,
		} &&
		self.organization.matches_view(&employee.organization) &&
		self.person.matches_view(&employee.person) &&
		self.title.matches(&employee.title) &&
		self.status.matches(&employee.status)
	}
}
