//! # Summary
//!
//! This crate provides definitions of what a user's `clinvoice` configuration should look like.

#![allow(clippy::tabs_in_doc_comments)]

mod config;
mod employees;
mod invoices;
mod jobs;
mod store_value;

pub use config::{Config, Error, Result};
pub use employees::Employees;
pub use invoices::Invoices;
pub use jobs::Jobs;
pub use store_value::StoreValue;
