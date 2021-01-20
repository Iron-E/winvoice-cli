use super::{AnyValue, Deletable, Updatable};
use crate::Store;
use clinvoice_data::{Id, Location};
use core::fmt::Display;
use std::error::Error;

pub trait LocationAdapter<'name, 'pass, 'path, 'user> :
	Deletable<'pass, 'path, 'user> +
	Display +
	Into<Location<'name>> +
	Into<Store<'pass, 'path, 'user>> +
	Updatable +
{
	/// # Summary
	///
	/// Create a new `Location` with a generated ID.
	///
	/// # Parameters
	///
	/// See [`Location`].
	///
	/// # Returns
	///
	/// ```ignore
	/// Location {name, id: /* generated */};
	/// ```
	fn create(name: &'_ str, store: Store<'pass, 'path, 'user>) -> Result<Self, Box<dyn Error>>;

	/// # Summary
	///
	/// Create a new [`Location`] which is inside of `self`.
	///
	/// # Parameters
	///
	/// See [`Location`].
	///
	/// # Returns
	///
	/// ```ignore
	/// Location {name, id: /* generated */, outside_id: self.unroll().id};
	/// ```
	fn create_inner(&self, name: &'_ str) -> Result<Self, Box<dyn Error>>;

	/// # Summary
	///
	/// Initialize the database for a given [`Store`].
	fn init(store: Store<'pass, 'path, 'user>) -> Result<(), Box<dyn Error>>;

	/// # Summary
	///
	/// Retrieve a [`Location`] from an active [`Store`](core::Store).
	///
	/// # Parameters
	///
	/// See [`Location`].
	///
	/// # Returns
	///
	/// * An [`Error`], when something goes wrong.
	/// * A list of matches, if there are any.
	fn retrieve<'arr>(
		id: AnyValue<Id>,
		name: AnyValue<&'_ str>,
		store: Store<'pass, 'path, 'user>,
	) -> Result<&'arr [Self], Box<dyn Error>>;
}
