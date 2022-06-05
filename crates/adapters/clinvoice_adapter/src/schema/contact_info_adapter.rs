use std::collections::HashMap;

use clinvoice_match::{MatchContact, MatchSet};
use clinvoice_schema::{Contact, ContactKind, Id};
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
		contact_info: Vec<(bool, ContactKind, String)>,
		employee_id: Id,
	) -> Result<Vec<Contact>>;

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
		match_condition: &MatchSet<MatchContact>,
	) -> Result<HashMap<Id, Vec<<Self as Deletable>::Entity>>>;
}
