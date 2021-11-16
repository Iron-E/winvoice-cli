use std::borrow::Cow;

use clinvoice_schema::Id;
#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

use super::{Match, MatchLocation, MatchStr};

/// # Summary
///
/// An [`Organization`](clinvoice_schema::Organization) with [matchable](Match) fields.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
pub struct MatchOrganization<'m>
{
	#[cfg_attr(feature = "serde_support", serde(default))]
	pub id: Match<'m, Id>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub location: MatchLocation<'m>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub name: MatchStr<Cow<'m, str>>,
}
