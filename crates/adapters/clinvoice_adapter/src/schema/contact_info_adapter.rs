use clinvoice_match::{MatchContact, MatchRow};
use clinvoice_schema::Contact;
use sqlx::{Executor, Pool, Result};

use crate::{Deletable, Updatable};

#[async_trait::async_trait]
pub trait ContactInfoAdapter:
	Deletable<Entity = Contact>
	+ Updatable<Db = <Self as Deletable>::Db, Entity = <Self as Deletable>::Entity>
{
	/// # Summary
	///
	/// Create new [`Contact`]s on the database.
	///
	/// # Parameters
	///
	/// `contact_info` is a slice of `(bool, ContactKind, String)`, which represents `(export, kind,
	/// label)` for the created [`Contact`]s.
	///
	/// # Returns
	///
	/// The newly created [`Contact`].
	async fn create(
		connection: impl 'async_trait + Executor<'_, Database = <Self as Deletable>::Db> + Send,
		contact_info: &[Contact],
	) -> Result<()>;

	/// # Summary
	///
	/// Retrieve some [`Contact`]s from the database using a [query](MatchContact).
	///
	/// # Returns
	///
	/// * An `Error`, if something goes wrong.
	/// * A list of matching [`Contact`]s.
	async fn retrieve(
		connection: &Pool<<Self as Deletable>::Db>,
		match_condition: &MatchRow<MatchContact>,
	) -> Result<Vec<<Self as Deletable>::Entity>>;
}
