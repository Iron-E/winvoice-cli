///! # Summary
///
/// This module provides definitions for what high-level views of each [data](super) item in this
/// crate should look like. These structures differ from their counterparts in typically obvious
/// ways (e.g. joins have been performed on relational data, and `Display` is implemented).

mod contact_view;
mod employee_view;
mod job_view;
mod location_view;
mod organization_view;
mod person_view;
mod restorable_serde;
mod timesheet_view;

pub use
{
	contact_view::ContactView,
	employee_view::EmployeeView,
	job_view::JobView,
	location_view::LocationView,
	organization_view::OrganizationView,
	person_view::PersonView,
	restorable_serde::RestorableSerde,
	timesheet_view::TimesheetView,
};
