//! # Summary
//!
//! This module implements adapters (and associated adapter types such as
//! [`Deletable`](clinvoice_adapter::data::Deletable)) for a Bincode filesystem.


mod employee;
mod error;
mod job;
mod location;
mod organization;
mod person;

pub use
{
	employee::BincodeEmployee,
	error::{Error, Result},
	job::BincodeJob,
	location::BincodeLocation,
	organization::BincodeOrganization,
	person::BincodePerson,
};
