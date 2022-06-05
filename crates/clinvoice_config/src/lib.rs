//! # Summary
//!
//! This crate provides definitions of what a user's `clinvoice` configuration should look like.

#[allow(clippy::tabs_in_doc_comments)]
mod config;
mod employees;
mod invoices;
mod store_value;
mod timesheets;

pub use config::{Config, Error, Result};
pub use employees::Employees;
pub use invoices::Invoices;
pub use store_value::StoreValue;
pub use timesheets::Timesheets;
