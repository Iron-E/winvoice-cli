use
{
	super::{MatchWhen, Deletable, Initializable, Updatable},
	crate::{DynamicResult, Store},
	clinvoice_data::{Location, Id, views::LocationView},
	core::fmt::Display,
	std::collections::HashSet,
};

pub trait LocationAdapter<'pass, 'path, 'user> :
	Deletable +
	Display +
	Initializable<'pass, 'path, 'user> +
	Into<Location> +
	Into<DynamicResult<LocationView>> +
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
	fn create<'name>(name: &'name str, store: Store<'pass, 'path, 'user>) -> DynamicResult<Self>;

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
	fn create_inner<'name>(&self, name: &'name str) -> DynamicResult<Self>;

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
	) -> DynamicResult<HashSet<Self>>;
}
