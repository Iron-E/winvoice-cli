use clinvoice_adapter::Store;
use serde::{
	Deserialize,
	Serialize,
};

/// # Summary
///
/// Possible values for the `[store]` field of the user config.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(untagged)]
pub enum StoreValue<'alias>
{
	/// # Summary
	///
	/// An alias of one ability name to another name.
	#[serde(borrow)]
	Alias(&'alias str),

	/// # Summary
	///
	/// A specification of storage.
	Storage(Store),
}
