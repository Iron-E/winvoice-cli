mod display;

#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

/// # Summary
///
/// Currently supported file systems / DBMS.
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
pub enum Adapters
{
	/// # Summary
	///
	/// A bincode filesystem.
	Postgres,
}
