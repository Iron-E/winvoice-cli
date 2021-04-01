//! # Summary
//!
//! This crate provides a definition for how functionality should be abstracted away from the
//! [`clinvoice` data](clinvoice_data). Every top-level type in [`clinvoice_data`] has a
//! corresponding Adapter type which must be implemented by aspiring permanent storages in order
//! for `clinvoice` to work with that storage scheme.
//!
//! # Usage
//!
//! 1. Begin by creating wrapper newtypes for each top-level data item in [`clinvoice_data`].
//!   * See the Bincode adapter for an example of this, or the [`Adapt`] macro for more information.
//! 2. Implement each newtype's corresponding `Adapter` trait.
//! 3. Create a new feature flag for the adapter on `clinvoice`.
//! 4. Write new `match` arms in `clinvoice`'s `Create` and `Retrieve` types for the adapter and
//!    conditionally compile them based on the feature flag.

mod adapters;
pub mod data;
mod error;
mod macros;
mod store;

pub use
{
	adapters::Adapters,
	error::Error,
	store::Store,
};
