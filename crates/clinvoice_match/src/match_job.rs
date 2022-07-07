mod exchangeable;
use core::time::Duration;

use clinvoice_schema::{chrono::NaiveDateTime, Id};
use humantime_serde::Serde;
#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

use super::{Match, MatchInvoice, MatchOrganization, MatchStr};
use crate::MatchOption;

/// # Summary
///
/// An [`Job`](clinvoice_schema::Job) with [matchable](Match) fields.
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct MatchJob
{
	#[cfg_attr(feature = "serde_support", serde(default))]
	pub client: MatchOrganization,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub date_close: MatchOption<NaiveDateTime>,

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
