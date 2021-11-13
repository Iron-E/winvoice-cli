use clinvoice_schema::{EmployeeStatus, Id};
#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

use super::{MatchContact, Match, MatchStr, MatchOrganization, MatchPerson};

/// # Summary
///
/// An [`Employee`](clinvoice_schema::Employee) with [matchable](Match) fields.
#[derive(Clone, Default, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
pub struct MatchEmployee<'m>
{
	#[cfg_attr(feature = "serde_support", serde(default))]
	pub contact_info: MatchContact<'m>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub id: Match<'m, Id>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub organization: MatchOrganization<'m>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub person: MatchPerson<'m>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub status: Match<'m, EmployeeStatus>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub title: MatchStr<String>,
}
