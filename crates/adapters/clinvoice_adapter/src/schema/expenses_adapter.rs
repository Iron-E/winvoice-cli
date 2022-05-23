use std::collections::HashMap;

use clinvoice_match::{MatchExpense, MatchSet};
use clinvoice_schema::{Expense, Id, Money};
use sqlx::{Executor, Pool, Result};

use crate::{Deletable, Updatable};

#[async_trait::async_trait]
pub trait ExpensesAdapter:
	Deletable<Entity = Expense>
	+ Updatable<Db = <Self as Deletable>::Db, Entity = <Self as Deletable>::Entity>
{
	/// # Summary
	///
	/// Create new [`Expense`]s on the database.
	///
	/// # Parameters
	///
	/// `expenses` is a slice of `(String, Money, String)`, which represents `(category, cost,
	/// description)` for the created [`Expense`]s.
	///
	/// # Returns
	///
	/// The newly created [`Expense`]s.
	async fn create(
		connection: impl 'async_trait + Executor<'_, Database = <Self as Deletable>::Db> + Send,
		expenses: Vec<(String, Money, String)>,
		timesheet_id: Id,
	) -> Result<Vec<Expense>>;

	/// # Summary
	///
	/// Retrieve some [`Expense`]s from the database using a [query](MatchExpense).
	///
	/// # Returns
	///
	/// * An `Error`, if something goes wrong.
	/// * A map of all matching [`Timesheet`](clinvoice_schema::Timesheet)s' [`Id`]s mapped to their
	///   respective [`Expense`]s.
	async fn retrieve(
		connection: &Pool<<Self as Deletable>::Db>,
		match_condition: MatchSet<MatchExpense>,
	) -> Result<HashMap<Id, Vec<<Self as Deletable>::Entity>>>;
}
