mod default;

#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

use super::MatchStr;
use crate::{MatchLocation, MatchRow};

/// # Summary
///
/// A [`Contact`](clinvoice_schema::Contact) with [matchable](Match) fields.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
pub enum MatchContactKind
{
	/// Same as [`Always`](crate::Match::Always).
	Any,

	/// Same as [`ContactKind::Address`](clinvoice_schema::ContactKind::Address).
	SomeAddress(#[cfg_attr(feature = "serde_support", serde(default))] MatchRow<MatchLocation>),

	/// Same as [`ContactKind::Email`](clinvoice_schema::ContactKind::Email).
	SomeEmail(#[cfg_attr(feature = "serde_support", serde(default))] MatchStr<String>),

	/// Same as [`ContactKind::Phone`](clinvoice_schema::ContactKind::Phone).
	SomePhone(#[cfg_attr(feature = "serde_support", serde(default))] MatchStr<String>),

	/// Same as [`ContactKind::Username`](clinvoice_schema::ContactKind::Username).
	SomeUsername(#[cfg_attr(feature = "serde_support", serde(default))] MatchStr<String>),

	/// Same as [`ContactKind::Wallet`](clinvoice_schema::ContactKind::Wallet).
	SomeWallet(#[cfg_attr(feature = "serde_support", serde(default))] MatchStr<String>),
}
