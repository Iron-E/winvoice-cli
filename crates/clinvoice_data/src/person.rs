use crate::{Contact, Id};

/// # Summary
///
/// A person is a physical human being.
pub struct Person<'contact_info, 'email, 'name, 'phone> where
	'email : 'contact_info,
	'phone : 'contact_info,
{
	/// # Summary
	///
	/// Contact information specific to the individual [`Person`], rather than a corporation they
	/// work at.
	pub contact_info: &'contact_info [Contact<'email, 'phone>],

	/// # Summary
	///
	/// This is the unique reference number for the [`Person`].
	pub id: Id,

	/// # Summary
	///
	/// This is the name of the [`Person`].
	pub name: &'name str,
}
