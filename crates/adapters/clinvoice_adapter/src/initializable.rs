use sqlx::{Acquire, Database, Result};

#[async_trait::async_trait]
pub trait Initializable
{
	type Db: Database;

	/// # Summary
	///
	/// Initialize the database for a given [`Store`].
	async fn init(
		connection: impl 'async_trait + Acquire<'_, Database = Self::Db> + Send,
	) -> Result<()>;
}
