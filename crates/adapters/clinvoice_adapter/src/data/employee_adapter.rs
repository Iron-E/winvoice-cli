#![allow(clippy::wrong_self_convention)]

use std::collections::HashMap;

use clinvoice_data::{
	views::EmployeeView,
	Contact,
	Employee,
	EmployeeStatus,
	Organization,
	Person,
};
use clinvoice_query as query;
use futures::Stream;
use sqlx::{Executor, Result};

use super::{Deletable, Updatable};

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
		connection: impl 'async_trait + Executor<'_, Database = <Self as Deletable>::Db>,
		contact_info: HashMap<String, Contact>,
		organization: Organization,
		person: Person,
		status: EmployeeStatus,
		title: String,
	) -> Result<<Self as Deletable>::Entity>;

	/// # Summary
	///
	/// Retrieve some [`Employee`]s from the database using a [query](query::Employee).
	///
	/// # Parameters
	///
	/// See [`Employee`].
	///
	/// # Returns
	///
	/// * Any matching [`Employee`]s.
	/// * An [`Error`], should something go wrong.
	fn retrieve<'a, S>(
		connection: impl Executor<'a, Database = <Self as Deletable>::Db>,
		query: &query::Employee,
	) -> S
	where
		S: Stream<Item = Result<<Self as Deletable>::Entity>>;

	/// # Summary
	///
	/// Retrieve some [`EmployeeView`]s from the database using a [query](query::Employee).
	///
	/// # Parameters
	///
	/// See [`Employee`].
	///
	/// # Returns
	///
	/// * Any matching [`Employee`]s.
	/// * An [`Error`], should something go wrong.
	fn retrieve_view<'a, S>(
		connection: impl Executor<'a, Database = <Self as Deletable>::Db>,
		query: &query::Employee,
	) -> S
	where
		S: Stream<Item = Result<EmployeeView>>;
}
