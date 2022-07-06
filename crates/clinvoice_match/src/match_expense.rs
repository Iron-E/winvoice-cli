mod exchangeable;

use clinvoice_schema::{Id, Money};
#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

use super::{Match, MatchStr};

/// A [`Expense`](clinvoice_schema::Expense) with [matchable](clinvoice_match) fields.
///
/// [`MatchExpense`] matches IFF all of its fields also match.
///
/// # Examples
///
/// ## YAML
///
/// Requires the `serde_support` feature. If any field is omitted, it will be set to the
/// [`Default`] for its type.
///
/// ```rust
/// # assert!(serde_yaml::from_str::<clinvoice_match::MatchExpense>(r#"
/// category:
///   regex: '^\s*([Ff]ood|[Tt]ravel)\s*$'
/// cost:
///   greater_than:
///     amount: "50.00"
///     currency: USD
/// description:
///   contains: "need"
/// id: any
/// timesheet_id:
///   equal_to: 4
/// # "#).is_ok());
/// ```
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct MatchExpense
{
	#[cfg_attr(feature = "serde_support", serde(default))]
	pub category: MatchStr<String>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub cost: Match<Money>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub description: MatchStr<String>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub id: Match<Id>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub timesheet_id: Match<Id>,
}
