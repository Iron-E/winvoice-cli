use std::error::Error;
use sqlx::{Database, Executor, Error as SqlxError};

/// # Summary
///
/// A structure which can be updated on some remote [`Store`][store].
#[async_trait::async_trait]
pub trait Updatable
{
	type Db: Database;
	type Entity;
	type Error: Error + From<SqlxError>;

	/// # Summary
	///
	/// Send this entity's data to the active [`Store`][store].
	///
	/// # Remarks
	///
	/// This function is called by create methods in order to write a generated entity to some
	/// [`Store`][store]. Manually creating an entity and running this function is not advised, as
	/// it does not guarantee the ID of an entity will be unique.
	///
	/// Rather, it is better to retrieve an entity or create one and then update it.
	///
	/// # Returns
	///
	/// * `()`, on a success.
	/// * An `Error`, when something goes wrong.
	///
	/// [store]: crate::Store
	async fn update(connection: impl 'async_trait + Executor<'_, Database = Self::Db>, entity: Self::Entity) -> Result<(), Self::Error>;
}
