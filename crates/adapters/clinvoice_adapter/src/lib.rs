//! # Summary
//!
//! This crate provides a definition for how functionality should be abstracted away from the
//! [`clinvoice` data](clinvoice_schema). Every top-level type in [`clinvoice_schema`] has a
//! corresponding Adapter type which must be implemented by aspiring permanent storages in order
//! for `clinvoice` to work with that storage scheme.
//!
//! # Usage
//!
//! 1. Begin by creating wrapper newtypes for each top-level data item in [`clinvoice_schema`].
//!    * See the Bincode adapter for an example of this, or the [`Adapt`] macro for more information.
//! 2. Implement each newtype's corresponding `Adapter` trait.
//! 3. Create a new feature flag for the adapter on `clinvoice`.
//! 4. Write new `match` arms in `clinvoice`'s `Create` and `Retrieve` types for the adapter and
//!    conditionally compile them based on the feature flag.

#![allow(clippy::unit_arg)]

pub mod fmt;
mod adapters;
mod deletable;
mod feature_not_found;
mod initializable;
pub mod schema;
mod store;
mod updatable;
mod write_where_clause;

pub use adapters::Adapters;
pub use deletable::Deletable;
pub use feature_not_found::{Error, Result};
pub use initializable::Initializable;
pub use store::Store;
pub use updatable::Updatable;
pub use write_where_clause::{WriteContext, WriteWhereClause};
