use sqlx::{Acquire, Database, Result};

/// Implementors of this trait are capable of creating an environment suitable for CLInvoice's
/// operation in the given [`Initializable::Db`].
#[async_trait::async_trait]
pub trait Initializable
{
	/// The [`Database`] environment in which CLInvoice will be initialized.
	type Db: Database;

	/// Initialize the [`Initializable::Db`] at the given `connection`.
	async fn init(
		connection: impl 'async_trait + Acquire<'_, Database = Self::Db> + Send,
	) -> Result<()>;
}
