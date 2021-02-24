use
{
	clinvoice_adapter::Store,
	serde::{Deserialize, Serialize},
};

/// # Summary
///
/// Possible values for the `[store]` field of the user config.
#[derive(Copy, Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(untagged)]
pub enum StoreValue<'alias, 'pass, 'path, 'user>
{
	/// # Summary
	///
	/// An alias of one ability name to another name.
	Alias(&'alias str),

	/// # Summary
	///
	/// A specification of storage.
	#[serde(borrow)]
	Storage(Store<'pass, 'path, 'user>),
}
