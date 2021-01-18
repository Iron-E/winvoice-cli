// LOCAL
mod any_value;
mod employee_adapter;
mod job_adapter;
mod location_adapter;
mod organization_adapter;
mod person_adapter;
mod deletable;
mod updatable;

pub use
{
	any_value::AnyValue,
	employee_adapter::EmployeeAdapter,
	job_adapter::JobAdapter,
	location_adapter::LocationAdapter,
	organization_adapter::OrganizationAdapter,
	person_adapter::PersonAdapter,
	deletable::Deletable,
	updatable::Updatable,
};
