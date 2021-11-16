use std::borrow::Cow;

use clinvoice_schema::{chrono::NaiveDateTime, Id};
#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

use super::{Match, MatchInvoice, MatchOrganization, MatchStr};

/// # Summary
///
/// An [`Job`](clinvoice_schema::Job) with [matchable](Match) fields.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
pub struct MatchJob<'m>
{
	#[cfg_attr(feature = "serde_support", serde(default))]
	pub client: MatchOrganization<'m>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub date_close: Match<'m, Option<NaiveDateTime>>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub date_open: Match<'m, NaiveDateTime>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub id: Match<'m, Id>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub invoice: MatchInvoice<'m>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub notes: MatchStr<Cow<'m, str>>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub objectives: MatchStr<Cow<'m, str>>,
}
