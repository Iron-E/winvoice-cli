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

#![allow(clippy::unit_arg)]

mod r#match;
mod match_employee;
mod match_expense;
mod match_invoice;
mod match_job;
mod match_location;
mod match_organization;
mod match_person;
mod match_str;
mod match_timesheet;

pub use humantime_serde::Serde;
pub use match_employee::MatchEmployee;
pub use match_expense::MatchExpense;
pub use match_invoice::MatchInvoice;
pub use match_job::MatchJob;
pub use match_location::{MatchLocation, MatchOuterLocation};
pub use match_organization::MatchOrganization;
pub use match_person::MatchPerson;
pub use match_str::MatchStr;
pub use match_timesheet::MatchTimesheet;
pub use r#match::Match;
