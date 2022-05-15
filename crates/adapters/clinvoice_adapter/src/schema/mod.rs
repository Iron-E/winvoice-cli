//! # Summary
//!
//! This module defines common and specific adapter types for [`clinvoice` data](clinvoice_schema).
//!
//! # Remarks
//!
//! One example of a common adapter type is [`Deletable`], since all top-level
//! [`clinvoice` data](clinvoice_schema) types may implement it with the same signature. This is in
//! contrast to [`JobAdapter`], which may only be implemented by [`Job`](clinvoice_schema::Job)s.

mod contact_info_adapter;
mod employee_adapter;
mod job_adapter;
mod location_adapter;
mod organization_adapter;
mod person_adapter;
mod timesheet_adapter;

pub use contact_info_adapter::ContactInfoAdapter;
pub use employee_adapter::EmployeeAdapter;
pub use job_adapter::JobAdapter;
pub use location_adapter::LocationAdapter;
pub use organization_adapter::OrganizationAdapter;
pub use person_adapter::PersonAdapter;
pub use timesheet_adapter::TimesheetAdapter;
