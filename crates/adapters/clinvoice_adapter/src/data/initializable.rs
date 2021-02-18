use crate::{DynamicResult, Store};

pub trait Initializable<'pass, 'path, 'user>
{
	/// # Summary
	///
	/// Initialize the database for a given [`Store`].
	fn init(store: &Store<'pass, 'path, 'user>) -> DynamicResult<()>;
}
