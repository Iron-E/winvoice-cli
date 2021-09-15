use std::error::Error;

/// # Summary
///
/// A structure which can be deleted from a remote [`Store`](crate::Store).
#[async_trait::async_trait]
pub trait Deletable
{
	type Entity;
	type Error: Error;
	type Pool: Clone + Send + Sync;

	/// # Summary
	///
	/// Delete a [`Person`].
	///
	/// # Paramteters
	///
	/// * `id`, the [`Id`] of the [`Person`] to delete.
	/// * `cascade`, whether or not to delete entries which reference this entity.
	///
	/// # Remarks
	///
	/// If `cascade` is false, the deletion operation will be restricted if any entities are found
	/// that require this one.
	///
	/// # Returns
	///
	/// * `()`, on a success.
	/// * An [`Error`] when:
	///   * `self.id` had not already been `create`d.
	///   * Something goes wrong.
	async fn delete(cascade: bool, entities: &[Self::Entity], pool: &Self::Pool) -> Result<(), Self::Error>;
}
