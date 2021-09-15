//! # Summary
//!
//! This module defines common and specific adapter types for [`clinvoice` data](clinvoice_data).
//!
//! # Remarks
//!
//! One example of a common adapter type is [`Deletable`], since all top-level
//! [`clinvoice` data](clinvoice_data) types may implement it with the same signature. This is in
//! contrast to [`JobAdapter`], which may only be implemented by [`Job`](clinvoice_data::Job)s.

mod deletable;
mod employee_adapter;
mod error;
mod initializable;
mod job_adapter;
mod location_adapter;
mod organization_adapter;
mod person_adapter;
mod updatable;

pub use deletable::Deletable;
pub use employee_adapter::EmployeeAdapter;
pub use error::Error;
pub use initializable::Initializable;
pub use job_adapter::JobAdapter;
pub use location_adapter::LocationAdapter;
pub use organization_adapter::OrganizationAdapter;
pub use person_adapter::PersonAdapter;
pub use updatable::Updatable;
