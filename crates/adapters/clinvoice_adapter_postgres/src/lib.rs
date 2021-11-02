//! # Summary
//!
//! This crate provides an implementation of [`clinvoice_adapter`] for a Postgres filesystem.

#![allow(clippy::from_over_into)]

pub mod schema;
pub use schema::PostgresSchema;
