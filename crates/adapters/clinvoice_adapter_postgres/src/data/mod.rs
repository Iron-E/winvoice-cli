//! # Summary
//!
//! This module implements adapters (and associated adapter types such as
//! [`Deletable`](clinvoice_adapter::data::Deletable)) for a Postgres filesystem.


mod employee;
mod error;
mod job;
mod location;
mod organization;
mod person;

pub use
{
	employee::PostgresEmployee,
	error::{Error, Result},
	job::PostgresJob,
	location::PostgresLocation,
	organization::PostgresOrganization,
	person::PostgresPerson,
};
