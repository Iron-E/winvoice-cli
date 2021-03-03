pub mod contact;
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
