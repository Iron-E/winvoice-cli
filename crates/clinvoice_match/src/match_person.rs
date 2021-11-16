use std::borrow::Cow;

use clinvoice_schema::Id;
#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

use super::{Match, MatchStr};

/// # Summary
///
/// An [`Location`](clinvoice_schema::Location) with [matchable](Match) fields.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
pub struct MatchPerson<'m>
{
	#[cfg_attr(feature = "serde_support", serde(default))]
	pub id: Match<'m, Id>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub name: MatchStr<Cow<'m, str>>,
}