//! This module defines adapters which enable the creation and retrieval of various data types
//! using a [`Database`][database]. Each adapter must implement [`Deletable`](crate::Deletable) and
//! [`Updatable`](crate::Updatable) for its respective data type.
//!
//! This module also has `const` representations of the [`columns`] for every table in the
//! [`Database`][database] as well.
//!
//! [database]: sqlx::Database

pub mod columns;
mod contact_info_adapter;
mod employee_adapter;
mod expenses_adapter;
mod job_adapter;
mod location_adapter;
mod organization_adapter;
mod timesheet_adapter;

pub use contact_info_adapter::ContactInfoAdapter;
pub use employee_adapter::EmployeeAdapter;
pub use expenses_adapter::ExpensesAdapter;
pub use job_adapter::JobAdapter;
pub use location_adapter::LocationAdapter;
pub use organization_adapter::OrganizationAdapter;
pub use timesheet_adapter::TimesheetAdapter;
