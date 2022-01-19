///! # Summary
///
/// This module provides definitions for what high-level views of each [data](super) item in this
/// crate should look like. These structures differ from their counterparts in typically obvious
/// ways (e.g. joins have been performed on relational data, and `Display` is implemented).
mod contact_view;
mod employee_view;
mod job_view;
mod location_view;
mod markdown;
mod organization_view;
mod person_view;
mod restorable_serde;
mod timesheet_view;

pub use contact_view::Contact;
pub use employee_view::Employee;
pub use job_view::Job;
pub use location_view::Location;
pub use organization_view::Organization;
pub use person_view::Person;
pub use restorable_serde::RestorableSerde;
pub use timesheet_view::Timesheet;
