use crate::{DynamicResult, Store};

pub trait Initializable
{
	/// # Summary
	///
	/// Initialize the database for a given [`Store`].
	fn init(store: &Store) -> DynamicResult<()>;
}
