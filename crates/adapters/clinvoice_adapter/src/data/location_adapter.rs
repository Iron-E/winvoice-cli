use
{
	super::{MatchWhen, Deletable, Initializable, Updatable},
	crate::Store,
	clinvoice_data::{Location, Id, views::LocationView},
	core::fmt::Display,
	std::{collections::HashSet, error::Error},
};

pub trait LocationAdapter<'pass, 'path, 'user> :
	Deletable +
	Display +
	Initializable<'pass, 'path, 'user> +
	Into<Location> +
	Into<Result<LocationView, Box<dyn Error>>> +
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
	fn create<'name>(name: &'name str, store: Store<'pass, 'path, 'user>) -> Result<Self, Box<dyn Error>>;

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
	fn create_inner<'name>(&self, name: &'name str) -> Result<Self, Box<dyn Error>>;

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
	fn retrieve(
		id: MatchWhen<Id>,
		name: MatchWhen<String>,
		outer: MatchWhen<Option<Id>>,
		store: Store<'pass, 'path, 'user>,
	) -> Result<HashSet<Self>, Box<dyn Error>>;
}
