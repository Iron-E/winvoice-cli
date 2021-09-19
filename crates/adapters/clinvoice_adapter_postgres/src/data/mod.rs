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

pub use schema::PostgresSchema;

pub use
{
	employee::PostgresEmployee,
	job::PostgresJob,
	location::PostgresLocation,
	organization::PostgresOrganization,
	person::PostgresPerson,
};
