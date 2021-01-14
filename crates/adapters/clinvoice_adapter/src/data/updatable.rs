use std::error::Error;

/// # Summary
///
/// A structure which can be updated on some remote [`Store`](crate::Store).
pub trait Updatable<'err>
{
	/// # Summary
	///
	/// Send this [`Person`]'s data to the active [`Store`](crate::Store).
	///
	/// # Returns
	///
	/// * `()`, on a success.
	/// * An `Error`, when:
	///   * `self.id` had not already been `create`d.
	///   * Something goes wrong.
	fn update(&self) -> Result<(), &'err dyn Error>;
}
