use clinvoice_match::MatchEmployee;
use clinvoice_schema::{ContactKind, Employee, Organization};
use sqlx::{Pool, Result};

use crate::{Deletable, Updatable};

#[async_trait::async_trait]
pub trait EmployeeAdapter:
	Deletable<Entity = Employee>
	+ Updatable<Db = <Self as Deletable>::Db, Entity = <Self as Deletable>::Entity>
{
	/// # Summary
	///
	/// Create some [`Employee`] on the database.
	///
	/// # Parameters
	///
	/// See [`Employee`].
	///
	/// # Returns
	///
	/// * The created [`Employee`], if there were no errors.
	/// * An [`Error`], if something goes wrong.
	async fn create(
		connection: &Pool<<Self as Deletable>::Db>,
		contact_info: Vec<(bool, ContactKind, String)>,
		name: String,
		organization: Organization,
		status: String,
		title: String,
	) -> Result<<Self as Deletable>::Entity>;

	/// # Summary
	///
	/// Retrieve some [`Employee`]s from the database using a [query](MatchEmployee).
	///
	/// # Parameters
	///
	/// See [`Employee`].
	///
	/// # Returns
	///
	/// * Any matching [`Employee`]s.
	/// * An [`Error`], should something go wrong.
	async fn retrieve(
		connection: &Pool<<Self as Deletable>::Db>,
		match_condition: MatchEmployee,
	) -> Result<Vec<<Self as Deletable>::Entity>>;
}
