//! # Summary
//!
//! This module contains [view](clinvoice_data::views)-like structures which are queries that
//! correspond to the [data item](clinvoice_data) of the same name.
//!
//! # Remarks
//!
//! Each field of each structure contains an identically-named, but [matchable](crate::data::Match)
//! field which should be used to specify the desired contents of the structure.
//!
//! # Example
//!
//! For examples, see the `retrieve` tests for each adapter below:
//!
//! * [`Employee`](crate::data::EmployeeAdapter)
//! * [`Job`](crate::data::JobAdapter)
//! * [`Location`](crate::data::LocationAdapter)
//! * [`Organization`](crate::data::OrganizationAdapter)
//! * [`Person`](crate::data::PersonAdapter)

#![allow(clippy::tabs_in_doc_comments)]

mod contact;
mod employee;
mod error;
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
pub use error::{Error, Result};
pub use expense::Expense;
pub use invoice::Invoice;
pub use job::Job;
pub use location::{Location, OuterLocation};
pub use match_str::MatchStr;
pub use organization::Organization;
pub use person::Person;
pub use r#match::Match;
pub use timesheet::Timesheet;
