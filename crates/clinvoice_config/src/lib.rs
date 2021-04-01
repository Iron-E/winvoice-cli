//! # Summary
//!
//! This crate provides definitions of what a user's `clinvoice` configuration should look like.

mod config;
mod employees;
mod invoices;
mod store_value;
mod timesheets;

pub use
{
	config::{Config, Error, Result},
	employees::Employees,
	invoices::Invoices,
	store_value::StoreValue,
	timesheets::Timesheets,
};
