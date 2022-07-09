//! This module holds types which represent the columns for every table that CLInvoice requires to
//! exist in a [`Database`](sqlx::Database).
//!
//! # Examples
//!
//! ```rust
//! use clinvoice_adapter::{
//!   fmt::{QueryBuilderExt, sql},
//!   schema::columns::LocationColumns,
//! };
//! use sqlx::{Execute, Postgres, QueryBuilder};
//!
//! let mut query = QueryBuilder::<Postgres>::new(sql::SELECT);
//!
//! assert_eq!(
//!   query
//!     .push_columns(&LocationColumns::default().default_scope().r#as(LocationColumns {
//!       id: "aliased_id",
//!       name: "aliased_name",
//!       outer_id: "aliased_outer_id",
//!     }))
//!     .prepare()
//!     .sql(),
//!   " SELECT L.id AS aliased_id,L.name AS aliased_name,L.outer_id AS aliased_outer_id;"
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
