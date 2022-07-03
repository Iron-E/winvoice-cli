#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

use super::MatchLocation;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
pub enum MatchOuterLocation
{
	/// # Summary
	///
	/// Always match.
	#[default]
	Any,

	/// # Summary
	///
	/// Match only when there is no [`outer_id`](clinvoice_schema::Location).
	None,

	/// # Summary
	///
	/// Match only when a specific [`outer_id`](clinvoice_schema::Location) resolves to a
	/// matching [`Location`].
	Some(Box<MatchLocation>),
}
