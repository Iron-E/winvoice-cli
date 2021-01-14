mod employee;
mod job;
mod location;
mod organization;
mod person;

pub use
{
	employee::TomlEmployee,
	job::TomlJob,
	location::TomlLocation,
	organization::TomlOrganization,
	person::TomlPerson,
};
