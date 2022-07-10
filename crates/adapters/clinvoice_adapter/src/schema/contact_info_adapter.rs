use clinvoice_match::MatchContact;
use clinvoice_schema::Contact;
use sqlx::{Executor, Pool, Result};

use crate::{Deletable, Updatable};

/// Implementors of this trait may act as an [adapter](super) for [`Contact`]s.
#[async_trait::async_trait]
pub trait ContactInfoAdapter:
	Deletable<Entity = Contact>
	+ Updatable<Db = <Self as Deletable>::Db, Entity = <Self as Deletable>::Entity>
{
	/// Initialize all of the [`Contact`]s in `contact_info` via the `connection`.
	///
	/// If you want to update an existing [`Contact`] instead, try [`Updatable::update`].
	async fn create(
		connection: impl 'async_trait + Executor<'_, Database = <Self as Deletable>::Db> + Send,
		contact_info: impl 'async_trait + Iterator<Item = &Contact> + Send,
	) -> Result<()>;

	/// Retrieve all [`Contact`]s (via `connection`) that match the `match_condition`.
	async fn retrieve(
		connection: &Pool<<Self as Deletable>::Db>,
		match_condition: &MatchContact,
	) -> Result<Vec<<Self as Deletable>::Entity>>;
}
