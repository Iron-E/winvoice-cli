mod display;

#[cfg(feature = "serde_support")]
use serde::{
	Deserialize,
	Serialize,
};

use crate::Error;

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
	Bincode,

	/// # Summary
	///
	/// A bincode filesystem.
	Postgres,
}

impl Adapters
{
	/// # Summary
	///
	/// Report an [`AdapterMismatchException`] due to `actual` being different than `self`.
	///
	/// # Parameters
	///
	/// * `actual`, the received adapter type.
	///
	/// # Returns
	///
	/// An [`AdapterMismatchException`] with a descriptive error message.
	pub fn mismatch(&self, actual: &Self) -> Result<(), Error>
	{
		if self != actual
		{
			return Err(Error::AdapterMismatch {
				expected: *self,
				actual:   *actual,
			});
		}

		Ok(())
	}
}
