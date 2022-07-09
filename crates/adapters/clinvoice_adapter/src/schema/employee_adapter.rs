use clinvoice_match::MatchEmployee;
use clinvoice_schema::Employee;
use sqlx::{Pool, Result};

use crate::{Deletable, Updatable};

/// Implementors of this trait may act as an [adapter](super) for [`Employee`]s.
#[async_trait::async_trait]
pub trait EmployeeAdapter:
	Deletable<Entity = Employee>
	+ Updatable<Db = <Self as Deletable>::Db, Entity = <Self as Deletable>::Entity>
{
	/// Initialize and return a new [`Employee`] via the `connection`.
	async fn create(
		connection: &Pool<<Self as Deletable>::Db>,
		name: String,
		status: String,
		title: String,
	) -> Result<<Self as Deletable>::Entity>;

	/// Retrieve all [`Employee`]s (via `connection`) that match the `match_condition`.
	async fn retrieve(
		connection: &Pool<<Self as Deletable>::Db>,
		match_condition: &MatchEmployee,
	) -> Result<Vec<<Self as Deletable>::Entity>>;
}
