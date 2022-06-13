//! # Summary
//!
//! This crate provides definitions of what a user's `clinvoice` configuration should look like.

#[allow(clippy::tabs_in_doc_comments)]

mod adapters;
mod config;
mod employees;
mod error;
mod invoices;
mod store;
mod store_value;
mod timesheets;

pub use adapters::Adapters;
pub use config::Config;
pub use employees::Employees;
pub use error::{Error, Result};
pub use invoices::Invoices;
pub use store::Store;
pub use store_value::StoreValue;
pub use timesheets::Timesheets;
