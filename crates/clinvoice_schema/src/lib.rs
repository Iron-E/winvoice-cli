//! This crate provides definitions for the information which is managed by CLInvoice. The data is
//! represented as it would be after all `JOIN`s are performed on a database (e.g. an
//! [`Organization`] in a database would likely reference [`Location`] by [`Id`], rather than
//! aggregating it).
//!
//! # Features
//!
//! * `serde_support` adds support for the [`serde`] crate.
//!
//! # Re-exports
//!
//! The crate provides access to the following elements of other crates:
//!
//! * Elements of the [`clinvoice_finance`] which are required to instantiate data (e.g. [`Money`]).
//! * The entire [`chrono`] crate, as almost all of it is required to instantiate certain data.

mod contact;
mod employee;
mod expense;
mod id;
mod invoice;
mod invoice_date;
mod job;
mod location;
mod organization;
mod restorable_serde;
mod restore_error;
mod timesheet;

pub use chrono;
pub use clinvoice_finance::{Currency, Decimal, Money};
pub use contact::{Contact, ContactKind};
pub use employee::Employee;
pub use expense::Expense;
pub use id::Id;
pub use invoice::Invoice;
pub use invoice_date::InvoiceDate;
pub use job::Job;
pub use location::Location;
pub use organization::Organization;
pub use restorable_serde::RestorableSerde;
pub use restore_error::{RestoreError, RestoreResult};
pub use timesheet::Timesheet;
