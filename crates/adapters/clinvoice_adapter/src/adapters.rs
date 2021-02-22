mod display;

use crate::Error;

/// # Summary
///
/// Currently supported file systems / DBMS.
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Adapters
{
	/// # Summary
	///
	/// A bincode filesystem.
	Bincode,
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
			return Err(Error::AdapterMismatch {expected: *self, actual: *actual});
		}

		return Ok(());
	}
}
