//! `clinvoice_match` contains types that have counterparts with identical layout in
//! [`clinvoice_schema`]. The only difference between the structures in this crate and
//! [`clinvoice_schema`] is that the types in this crate can be used to describe any number of their
//! counterpart types.
//!
//! The ability to "describe" other types comes from [`Match`], [`MatchSet`], and [`MatchStr`].
//! As this is the distinguishing feature of the crate, none of those three types have equivalents
//! in [`clinvoice_schema`].
//!
//! # Features
//!
//! * `serde_support` adds support for the [`serde`] crate. This crate is intended for and tested
//!   with [`serde_yaml`](https://docs.serde.rs/serde_yaml/) in particular.
//!
//! # Re-exports
//!
//! This crate re-exports [`humantime_serde::Serde`], as it is required to deserialize the
//! `increment` of a [`MatchJob`] via human-readable time (e.g. "15min").
//!
//! # Examples
//!
//! The following [`MatchEmployee`] represents all [`Employee`](clinvoice_schema::Employee)s who
//! meet all of the following criteria:
//!
//! * Have a `name` starting with 'A', 'B', or 'C'.
//! * Have a `status` equal to "Hired".
//! * Have a `title` not equal to "CEO".
//!
//! ```rust
//! use clinvoice_match::{Match, MatchEmployee, MatchStr};
//!
//! let _ = MatchEmployee {
//!   name: MatchStr::Regex("^[ABC]".into()),
//!   status: "Hired".to_string().into(),
//!   title: MatchStr::Not(Box::new("CEO".to_string().into())),
//!   ..Default::default()
//! };
//! ```

mod r#match;
mod match_contact;
mod match_employee;
mod match_expense;
mod match_invoice;
mod match_job;
mod match_location;
mod match_option;
mod match_organization;
mod match_set;
mod match_str;
mod match_timesheet;

pub use humantime_serde::Serde;
pub use match_contact::{MatchContact, MatchContactKind};
pub use match_employee::MatchEmployee;
pub use match_expense::MatchExpense;
pub use match_invoice::MatchInvoice;
pub use match_job::MatchJob;
pub use match_location::{MatchLocation, MatchOuterLocation};
pub use match_option::MatchOption;
pub use match_organization::MatchOrganization;
pub use match_set::MatchSet;
pub use match_str::MatchStr;
pub use match_timesheet::MatchTimesheet;
pub use r#match::Match;
