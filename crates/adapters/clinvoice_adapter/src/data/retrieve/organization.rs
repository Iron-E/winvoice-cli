use
{
	super::Location,
	crate::data::MatchWhen,
	clinvoice_data::Id,
	serde::{Deserialize, Serialize},
};

/// # Summary
///
/// An [`Organization`](clinvoice_data::Organization) with [matchable](MatchWhen) fields.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature="serde_support", derive(Deserialize, Serialize))]
pub struct Organization<'m>
{
	#[cfg_attr(feature="serde_support", serde(default))]
	pub id: MatchWhen<'m, Id>,

	#[cfg_attr(feature="serde_support", serde(default))]
	pub location: Location<'m>,

	#[cfg_attr(feature="serde_support", serde(default))]
	pub name: MatchWhen<'m, String>,
}
