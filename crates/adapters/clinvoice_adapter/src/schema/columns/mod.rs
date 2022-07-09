//! This module holds types which represent the columns for every table that CLInvoice requires to
//! exist in a [`Database`](sqlx::Database).
//!
//! # Examples
//!
//! ```rust
//! use clinvoice_adapter::{
//!   fmt::{QueryBuilderExt, sql},
//!   schema::columns::OrganizationColumns,
//! };
//! # use pretty_assertions::assert_eq;
//! use sqlx::{Execute, Postgres, QueryBuilder};
//!
//! let mut query = QueryBuilder::<Postgres>::new(sql::SELECT);
//!
//! assert_eq!(
//!   query
//!     .push_columns(&OrganizationColumns::default().default_scope().r#as(OrganizationColumns {
//!       id: "aliased_id",
//!       location_id: "aliased_location_id",
//!       name: "aliased_name",
//!     }))
//!     .prepare()
//!     .sql(),
//!   " SELECT O.id AS aliased_id,O.location_id AS aliased_location_id,O.name AS aliased_name;"
//! );
//! ```

mod contact;
mod employee;
mod expense;
mod job;
mod location;
mod organization;
mod timesheet;

pub use contact::ContactColumns;
pub use employee::EmployeeColumns;
pub use expense::ExpenseColumns;
pub use job::JobColumns;
pub use location::LocationColumns;
pub use organization::OrganizationColumns;
pub use timesheet::TimesheetColumns;
