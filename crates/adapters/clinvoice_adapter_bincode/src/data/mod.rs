pub mod contact;
mod employee;
mod job;
mod location;
mod organization;
mod person;

pub use
{
	employee::BincodeEmployee,
	job::BincodeJob,
	location::BincodeLocation,
	organization::BincodeOrganization,
	person::BincodePerson,
};
