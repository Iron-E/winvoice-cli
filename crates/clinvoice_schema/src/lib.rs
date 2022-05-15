//! # Summary
//!
//! This crate provides a complete resource for data items which are to be stored in a database (or
//! other permanent storage fixture). It is only dependent on `clinvoice_finance` for currency
//! conversions, and is otherwise independent of the CLInvoice project.
//!
//! Consequently, most other parts of `clinvoice` depend on this crate.
//!
//! # Features
//!
//! Support for [`serde`](http://serde.rs/) can be enabled with the `serde_support` feature flag.
//! Otherwise, serialization will have to be implemented for these types by hand.
//!
//! # Remarks
//!
//! In the base you can find the types which are intended to be stored (e.g. [`Contact`]) and in
//! [`views`] you can find all logical views of the data.

#![allow(clippy::unit_arg)]

mod contact;
mod employee;
mod expense;
mod from_str_error;
mod id;
mod invoice;
mod invoice_date;
mod job;
mod location;
mod markdown;
mod organization;
mod person;
mod restorable_serde;
mod restore_error;
mod timesheet;

pub use chrono;
pub use clinvoice_finance::{Currency, Decimal, Money};
pub use contact::{Contact, ContactKind};
pub use employee::Employee;
pub use expense::Expense;
pub use from_str_error::{FromStrError, FromStrResult};
pub use id::Id;
pub use invoice::Invoice;
pub use invoice_date::InvoiceDate;
pub use job::Job;
pub use location::Location;
pub use organization::Organization;
pub use person::Person;
pub use restorable_serde::RestorableSerde;
pub use restore_error::{RestoreError, RestoreResult};
pub use timesheet::Timesheet;
