//! # Summary
//!
//! This module contains [view](clinvoice_schema::views)-like structures which are queries that
//! correspond to the [data item](clinvoice_schema) of the same name.
//!
//! # Remarks
//!
//! Each field of each structure contains an identically-named, but [matchable](crate::schema::Match)
//! field which should be used to specify the desired contents of the structure.
//!
//! # Example
//!
//! For examples, see the `retrieve` tests for each adapter below:
//!
//! * [`Employee`](crate::schema::EmployeeAdapter)
//! * [`Job`](crate::schema::JobAdapter)
//! * [`Location`](crate::schema::LocationAdapter)
//! * [`Organization`](crate::schema::OrganizationAdapter)
//! * [`Person`](crate::schema::PersonAdapter)

#![allow(clippy::tabs_in_doc_comments)]

mod contact;
mod employee;
mod expense;
mod invoice;
mod job;
mod location;
mod r#match;
mod match_str;
mod organization;
mod person;
mod timesheet;

pub use contact::Contact;
pub use employee::Employee;
pub use expense::Expense;
pub use invoice::Invoice;
pub use job::Job;
pub use location::{Location, OuterLocation};
pub use match_str::MatchStr;
pub use organization::Organization;
pub use person::Person;
pub use r#match::Match;
pub use timesheet::Timesheet;
