mod display;

use crate::AdapterMismatchError;

/// # Summary
///
/// Currently supported file systems / DBMS.
#[derive(Debug, PartialEq)]
pub enum Adapters
{
	/// # Summary
	///
	/// A TOML filesystem.
	TOML,
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
	pub fn mismatch<'msg>(&self, actual: &Self) -> AdapterMismatchError<'msg>
	{
		return AdapterMismatchError
		{
			message: format!("Expected the {} adapter, but got the {} adapter.", self, actual).into(),
		};
	}
}
