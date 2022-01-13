#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

use super::{MatchLocation, MatchStr};

/// # Summary
///
/// A method through which something can be communicated with.
#[derive(Clone, Default, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
pub struct MatchContact
{
	#[cfg_attr(feature = "serde_support", serde(default))]
	pub address: MatchLocation,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub email: MatchStr<String>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub phone: MatchStr<String>,
}
