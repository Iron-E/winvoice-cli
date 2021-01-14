use super::{AnyValue, Deletable, Updatable};
use clinvoice_data::{Id, Location};
use core::fmt::Display;
use std::error::Error;

pub trait CrudLocation<'err, 'name> :
	Deletable<'err> +
	Display +
	From<Location<'name>> +
	Updatable<'err> +
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
	fn create(name: &'_ str) -> Result<Self, &'err dyn Error>;

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
	fn create_inner(&self, name: &'_ str) -> Result<Self, &'err dyn Error>;

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
	fn retrieve<'arr>(id: AnyValue<Id>, name: AnyValue<&'_ str>) -> Result<&'arr [Self], &'err dyn Error>;
}
