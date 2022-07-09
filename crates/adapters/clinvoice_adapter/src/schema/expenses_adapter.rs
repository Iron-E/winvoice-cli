use clinvoice_match::MatchExpense;
use clinvoice_schema::{Expense, Id, Money};
use sqlx::{Executor, Pool, Result};

use crate::{Deletable, Updatable};

/// Implementors of this trait may act as an [adapter](super) for [`Employee`]s.
#[async_trait::async_trait]
pub trait ExpensesAdapter:
	Deletable<Entity = Expense>
	+ Updatable<Db = <Self as Deletable>::Db, Entity = <Self as Deletable>::Entity>
{
	/// Initialize and return new [`Expense`]s via the `connection`.
	///
	/// # Parameters
	///
	/// `expenses` is a slice of `(String, Money, String)`, which represents `(category, cost,
	/// description)` for the created [`Expense`]s.
	async fn create(
		connection: impl 'async_trait + Executor<'_, Database = <Self as Deletable>::Db> + Send,
		expenses: Vec<(String, Money, String)>,
		timesheet_id: Id,
	) -> Result<Vec<Expense>>;

	/// Retrieve all [`Employee`]s (via `connection`) that match the `match_condition`.
	async fn retrieve(
		connection: &Pool<<Self as Deletable>::Db>,
		match_condition: &MatchExpense,
	) -> Result<Vec<<Self as Deletable>::Entity>>;
}
