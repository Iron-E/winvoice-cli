use crate::Id;

/// # Summary
///
/// A method through which something can be communicated with.
#[derive(Debug)]
pub enum Contact<'email, 'phone>
{
	/// # Summary
	///
	/// A [`Location`](crate::Location).
	Address(Id),

	/// # Summary
	///
	/// An email address.
	///
	/// # Example
	///
	/// * 'foo@bar.io'
	Email(&'email str),

	/// # Summary
	///
	/// A phone number.
	///
	/// # Example
	///
	/// The following are valid for numbers with country code:
	///
	/// * '+1 (603) 555-1234'
	/// * '1-603-555-1234'
	/// * '16035551234'
	///
	/// The following are valid for numbers without country code:
	///
	/// * '(603) 555-1234'
	/// * '603-555-1234'
	/// * '6035551234'
	Phone(&'phone str),
}
