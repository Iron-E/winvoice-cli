// LOCAL
mod deletable;
mod employee_adapter;
mod job_adapter;
mod location_adapter;
mod match_when;
mod organization_adapter;
mod person_adapter;
mod updatable;

pub use
{
	deletable::Deletable,
	employee_adapter::EmployeeAdapter,
	job_adapter::JobAdapter,
	location_adapter::LocationAdapter,
	match_when::MatchWhen,
	organization_adapter::OrganizationAdapter,
	person_adapter::PersonAdapter,
	updatable::Updatable,
};
