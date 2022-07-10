//! This crate provides traits which can are used to provide an abstraction for CLInvoice frontends
//! to enable the option of different types of storage facilities (e.g. Postgres vs MySQL)
//!
//! Contains various tools, such as those to [generate](clinvoice_adapter::fmt) SQL and [reference
//! table columns](clinvoice_adapter::schema::columns).
//!
//! # Usage
//!
//! If you are looking to create a new adapter:
//!
//! 1. Create newtypes for each trait in [`clinvoice_adapter::schema`].
//! 2. Implement each newtype's corresponding `Adapter` trait.
//! 3. Add a new variant in [`Adapters`][adapters].
//! 4. Create a new feature flag for the adapter in the corresponding frontend you want to support
//!    the new adapter.
//! 5. Write add new `match` arms in areas that `match` on [`Adapters`][adapters].
//!
//! [adapters]: clinvoice_config::Adapters

mod deletable;
pub mod fmt;
mod initializable;
pub mod schema;
mod updatable;
mod write_where_clause;

pub use deletable::Deletable;
pub use initializable::Initializable;
pub use updatable::Updatable;
pub use write_where_clause::{WriteContext, WriteWhereClause};
