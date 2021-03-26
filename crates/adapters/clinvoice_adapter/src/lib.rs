#![allow(clippy::too_many_arguments)]

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
