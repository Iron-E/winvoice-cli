//! # Summary
//!
//! This module defines common and specific adapter types for [`clinvoice` data](clinvoice_data).
//!
//! # Remarks
//!
//! One example of a common adapter type is [`Deletable`], since all top-level
//! [`clinvoice` data](clinvoice_data) types may implement it with the same signature. This is in
//! contrast to [`JobAdapter`], which may only be implemented by [`Job`](clinvoice_data::Job)s.

pub mod contact;
mod deletable;
mod employee_adapter;
mod error;
mod initializable;
mod job_adapter;
mod location_adapter;
mod organization_adapter;
mod person_adapter;
mod updatable;
pub mod timesheet;

pub use
{
	deletable::Deletable,
	employee_adapter::EmployeeAdapter,
	error::Error,
	initializable::Initializable,
	job_adapter::JobAdapter,
	location_adapter::LocationAdapter,
	organization_adapter::OrganizationAdapter,
	person_adapter::PersonAdapter,
	updatable::Updatable,
};
