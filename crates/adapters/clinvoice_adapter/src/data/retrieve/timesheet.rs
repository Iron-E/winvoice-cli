use
{
	super::Employee,
	crate::data::MatchWhen,
	clinvoice_data::chrono::{DateTime, Utc},
	serde::{Deserialize, Serialize},
};

/// # Summary
///
/// An [`Timesheet`](clinvoice_data::Timesheet) with [matchable](MatchWhen) fields.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature="serde_support", derive(Deserialize, Serialize))]
pub struct Timesheet<'m>
{
	#[cfg_attr(feature="serde_support", serde(default))]
	pub begin: MatchWhen<'m, DateTime<Utc>>,

	#[cfg_attr(feature="serde_support", serde(default))]
	pub employee: Employee<'m>,

	#[cfg_attr(feature="serde_support", serde(default))]
	pub end: MatchWhen<'m, Option<DateTime<Utc>>>,
}
