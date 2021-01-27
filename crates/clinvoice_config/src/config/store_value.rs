use clinvoice_adapter::Store;

/// # Summary
///
/// Possible values for the `[store]` field of the user config.
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum StoreValue<'alias, 'pass, 'path, 'user>
{
	/// # Summary
	///
	/// An alias of one ability name to another name.
	Alias(&'alias str),

	/// # Summary
	///
	/// A specification of storage.
	Storage(Store<'pass, 'path, 'user>),
}
