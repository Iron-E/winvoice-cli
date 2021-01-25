use crate::{Contact, Id};
use serde::{Deserialize, Serialize};

/// # Summary
///
/// A person is a physical human being.
#[derive(Debug, Deserialize, Serialize)]
pub struct Person<'email, 'name, 'phone>
{
	/// # Summary
	///
	/// Contact information specific to the individual [`Person`], rather than a corporation they
	/// work at.
	pub contact_info: Vec<Contact<'email, 'phone>>,

	/// # Summary
	///
	/// This is the unique reference number for the [`Person`].
	pub id: Id,

	/// # Summary
	///
	/// This is the name of the [`Person`].
	pub name: &'name str,
}
