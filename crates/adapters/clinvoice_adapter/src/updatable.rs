use sqlx::{Database, Result, Transaction};

/// Implementors of this trait are capable of syncing the current state of an
/// [`Updatable::Entity`] with its mirror in the [`Updatable::Db`].
#[async_trait::async_trait]
pub trait Updatable
{
	/// The [`Database`] where data of type [`Updatable::Entity`] is being stored.
	type Db: Database;

	/// The type of data that is to be [`update`](Deletable::update)d.
	type Entity;

	/// Update each [`Updatable::Entity`] in `entities` via the `connection`.
	///
	/// # Errors
	///
	/// * If any [`Updatable::Entity`] in `entities` does not exist over the `connection`.
	async fn update<'e, 'i>(
		connection: &mut Transaction<Self::Db>,
		entities: impl 'async_trait + Clone + Iterator<Item = &'i Self::Entity> + Send,
	) -> Result<()>
	where
		'e: 'i,
		Self::Entity: 'e;
}
