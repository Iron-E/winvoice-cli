use std::error::Error;
use sqlx::{Database, Executor, Error as SqlxError};

/// # Summary
///
/// A structure which can be deleted from a remote [`Store`](crate::Store).
#[async_trait::async_trait]
pub trait Deletable
{
	type Db: Database;
	type Entity;
	type Error: Error + From<SqlxError>;

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
	///
	/// TODO: replace generics with `impl` after `async fn` stabilized
	async fn delete(
		connection: impl 'async_trait + Executor<'_, Database = Self::Db>,
		cascade: bool,
		entities: impl 'async_trait + Iterator<Item = Self::Entity> + Send,
	) -> Result<(), Self::Error>;
}
