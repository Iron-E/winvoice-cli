use crate::{Contact, Id};

/// # Summary
///
/// A person is a physical human being.
pub struct Person<'addr, 'contact_info, 'email, 'name>
{
	/// # Summary
	///
	/// Contact information specific to the [`Organization`] that the [`Employee`] does work for.
	pub contact_info: &'contact_info [Contact<'addr, 'email,>],

	/// # Summary
	///
	/// This is the unique reference number for the [`Person`].
	pub id: Id,

	/// # Summary
	///
	/// This is the name of the [`Person`].
	pub name: &'name str,
}
