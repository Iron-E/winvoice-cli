use clinvoice_data::{EmployeeStatus, Id};
#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

use super::{Contact, Match, MatchStr, Organization, Person};

/// # Summary
///
/// An [`Employee`](clinvoice_data::Employee) with [matchable](Match) fields.
#[derive(Clone, Default, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
pub struct Employee<'m>
{
	#[cfg_attr(feature = "serde_support", serde(default))]
	pub contact_info: Contact<'m>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub id: Match<'m, Id>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub organization: Organization<'m>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub person: Person<'m>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub status: Match<'m, EmployeeStatus>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub title: MatchStr<String>,
}
