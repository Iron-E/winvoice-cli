mod exchangeable;
use core::time::Duration;

use clinvoice_schema::{chrono::NaiveDateTime, Id};
use humantime_serde::Serde;
#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

use super::{Match, MatchInvoice, MatchOrganization, MatchStr};
use crate::MatchOption;

/// A [`Job`](clinvoice_schema::Job) with [matchable](clinvoice_match) fields.
///
/// [`MatchJob`] matches IFF all of its fields also match.
///
/// # Notes
///
/// * See [`humantime_serde`] for the syntax of matched data in the `increment` field.
///
/// # Examples
///
/// ## YAML
///
/// Requires the `serde_support` feature. If any field is omitted, it will be set to the
/// [`Default`] for its type.
///
/// ```rust
/// # assert!(serde_yaml::from_str::<clinvoice_match::MatchJob>(r#"
/// client:
///   location:
///     name:
///       contains: "New"
/// date_close: none
/// date_open:
///   in_range: ["2022-05-01T00:00:00", "2022-05-02T00:00:00"]
/// id: any
/// increment:
///   equal_to: "5min"
/// invoice:
///   date_paid: none
///   date_issued: none
/// notes:
///   contains: |
///     here is some multiline text.
///     and some more text.
/// objectives: any
/// # "#).is_ok());
/// ```
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
