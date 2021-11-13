mod default;

#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

use super::MatchLocation;

#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
pub enum MatchOuterLocation<'m>
{
	Any,
	None,
	Some(Box<MatchLocation<'m>>),
}
