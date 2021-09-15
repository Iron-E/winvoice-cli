#![allow(clippy::wrong_self_convention)]

use std::{collections::HashMap, error::Error};

use clinvoice_data::{
	views::EmployeeView,
	Contact,
	Employee,
	EmployeeStatus,
	Organization,
	Person,
};
use clinvoice_query as query;

use super::{
	Deletable,
	Updatable,
};

#[async_trait::async_trait]
pub trait EmployeeAdapter:
	Deletable<Error = <Self as EmployeeAdapter>::Error>
	+ Updatable<Error = <Self as EmployeeAdapter>::Error>
{
	type Error: From<super::Error> + Error;

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
		contact_info: HashMap<String, Contact>,
		organization: Organization,
		person: Person,
		status: EmployeeStatus,
		title: String,
		pool: Self::Pool,
	) -> Result<Employee, <Self as EmployeeAdapter>::Error>;

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
	async fn retrieve(
		query: &query::Employee,
		pool: Self::Pool,
	) -> Result<Vec<Employee>, <Self as EmployeeAdapter>::Error>;

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
	async fn retrieve_view(
		query: &query::Employee,
		pool: Self::Pool,
	) -> Result<Vec<EmployeeView>, <Self as EmployeeAdapter>::Error>;
}
