use std::collections::HashMap;

use clinvoice_schema::{Id, Contact};
#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

use crate::MatchContact;

use super::{Match, MatchOrganization, MatchPerson, MatchStr};

/// # Summary
///
/// An [`Employee`](clinvoice_schema::Employee) with [matchable](Match) fields.
#[derive(Clone, Default, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
pub struct MatchEmployee
{
	#[cfg_attr(feature = "serde_support", serde(default))]
	pub contact_info: HashMap<MatchStr<String>, MatchContact>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub id: Match<Id>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub organization: MatchOrganization,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub person: MatchPerson,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub status: MatchStr<String>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub title: MatchStr<String>,
}
