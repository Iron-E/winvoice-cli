use std::error::Error;

/// # Summary
///
/// A structure which can be deleted from a remote [`Store`](crate::Store).
pub trait Deletable
{
	type Error : Error;

	/// # Summary
	///
	/// Delete a [`Person`].
	///
	/// # Paramteters
	///
	/// * `id`, the [`Id`] of the [`Person`] to delete.
	/// * `cascade`, whether or not to delete entries which reference this [`Person`].
	///
	/// # Returns
	///
	/// * `()`, on a success.
	/// * An [`Error`] when:
	///   * `self.id` had not already been `create`d.
	///   * Something goes wrong.
	fn delete(&self, cascade: bool) -> Result<(), Self::Error>;
}
