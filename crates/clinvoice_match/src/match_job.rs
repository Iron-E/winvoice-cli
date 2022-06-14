mod exchangeable;
use core::time::Duration;

use clinvoice_schema::{chrono::NaiveDateTime, Id};
use humantime_serde::Serde;
#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

use super::{Match, MatchInvoice, MatchOrganization, MatchStr};
use crate::MatchRow;

/// # Summary
///
/// An [`Job`](clinvoice_schema::Job) with [matchable](Match) fields.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
pub struct MatchJob
{
	#[cfg_attr(feature = "serde_support", serde(default))]
	pub client: MatchRow<MatchOrganization>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub date_close: Match<Option<NaiveDateTime>>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub date_open: Match<NaiveDateTime>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub id: Match<Id>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub increment: Match<Serde<Duration>>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub invoice: MatchInvoice,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub notes: MatchStr<String>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub objectives: MatchStr<String>,
}
