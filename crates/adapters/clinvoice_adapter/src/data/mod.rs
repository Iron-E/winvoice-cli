// LOCAL
mod any_value;
mod crud_employee;
mod crud_job;
mod crud_location;
mod crud_organization;
mod crud_person;
mod deletable;
mod updatable;

pub use
{
	any_value::AnyValue,
	crud_employee::CrudEmployee,
	crud_job::CrudJob,
	crud_location::CrudLocation,
	crud_organization::CrudOrganization,
	crud_person::CrudPerson,
	deletable::Deletable,
	updatable::Updatable,
};
