use
{
	super::{Organization, Person},
	crate::data::MatchWhen,
	clinvoice_data::{Contact, EmployeeStatus, Id},
	serde::{Deserialize, Serialize},
};

/// # Summary
///
/// An [`Employee`](clinvoice_data::Employee) with [matchable](MatchWhen) fields.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature="serde_support", derive(Deserialize, Serialize))]
pub struct Employee<'m>
{
	#[cfg_attr(feature="serde_support", serde(default))]
	pub contact_info: MatchWhen<'m, Contact>,

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
