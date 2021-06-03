//! # Summary
//!
//! This crate provides a complete resource for data items which are to be stored in a database (or
//! other permanent storage fixture). It is independent of all other crates in the `clinvoice`
//! suite, so the definitions can be used apart from any other project-local crate.
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

#![allow(clippy::suspicious_else_formatting)]

mod contact;
mod currency;
mod employee;
mod employee_status;
mod expense;
mod expense_category;
mod id;
mod invoice;
mod invoice_date;
mod job;
mod location;
mod money;
mod organization;
mod person;
mod timesheet;
pub mod views;

pub use
{
	contact::Contact,
	currency::Currency,
	employee::Employee,
	employee_status::EmployeeStatus,
	expense::Expense,
	expense_category::ExpenseCategory,
	id::Id,
	invoice::Invoice,
	invoice_date::InvoiceDate,
	job::Job,
	location::Location,
	money::Money,
	organization::Organization,
	person::Person,
	timesheet::Timesheet,
};

pub use chrono;
pub use rust_decimal::Decimal;

/// # Summary
///
/// The namespace for a v5 [`Uuid`](uuid::Uuid) containing CLInvoice data.
pub const UUID_NAMESPACE: Id = Id::from_bytes([
	0x1a, 0x88, 0xb1, 0xde,
	0xe8, 0x0d, 0x4e, 0xca,
	0x92, 0x08, 0xe5, 0x6b,
	0x09, 0x9a, 0x6f, 0x4b
]);
