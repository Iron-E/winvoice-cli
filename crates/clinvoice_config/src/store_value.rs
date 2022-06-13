use serde::{Deserialize, Serialize};

use crate::Store;

/// # Summary
///
/// Possible values for the `[store]` field of the user config.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(untagged)]
pub enum StoreValue
{
	/// # Summary
	///
	/// An alias of one ability name to another name.
	Alias(String),

	/// # Summary
	///
	/// A specification of storage.
	Storage(Store),
}
