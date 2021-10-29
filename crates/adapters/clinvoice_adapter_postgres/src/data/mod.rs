//! # Summary
//!
//! This module implements adapters (and associated adapter types such as
//! [`Deletable`](clinvoice_adapter::data::Deletable)) for a Postgres filesystem.

mod employee;
mod job;
mod location;
mod organization;
mod person;
mod schema;
mod timesheet;
#[cfg(test)]
mod util;

pub use employee::PostgresEmployee;
pub use job::PostgresJob;
pub use location::PostgresLocation;
pub use organization::PostgresOrganization;
pub use person::PostgresPerson;
pub use schema::PostgresSchema;
pub use timesheet::PostgresTimesheet;
