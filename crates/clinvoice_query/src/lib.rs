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

mod contact;
mod employee;
mod error;
mod expense;
mod invoice;
mod job;
mod location;
mod match_str;
mod organization;
mod person;
mod r#match;
mod timesheet;

pub use
{
	contact::Contact,
	employee::Employee,
	error::{Error, Result},
	expense::Expense,
	invoice::Invoice,
	job::Job,
	location::{Location, OuterLocation},
	r#match::Match,
	match_str::MatchStr,
	organization::Organization,
	person::Person,
	timesheet::Timesheet,
};
