use
{
	crate::data::MatchWhen,
	clinvoice_data::Id,
	serde::{Deserialize, Serialize},
};

/// # Summary
///
/// An [`Location`](clinvoice_data::Location) with [matchable](MatchWhen) fields.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature="serde_support", derive(Deserialize, Serialize))]
pub struct Location<'m>
{
	#[cfg_attr(feature="serde_support", serde(default))]
	pub id: MatchWhen<'m, Id>,

	#[cfg_attr(feature="serde_support", serde(default))]
	pub outer: Option<Box<Self>>,

	#[cfg_attr(feature="serde_support", serde(default))]
	pub name: MatchWhen<'m, String>,
}
