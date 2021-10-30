mod default;

#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

use super::Location;

#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
pub enum OuterLocation<'m>
{
	Any,
	None,
	Some(Box<Location<'m>>),
}
