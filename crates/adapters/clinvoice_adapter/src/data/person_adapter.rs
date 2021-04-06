use
{
	std::error::Error,

	super::{Deletable, Initializable, query, Updatable},
	crate::Store,

	clinvoice_data::Person,
};

pub trait PersonAdapter<'store> :
	Deletable<Error=<Self as PersonAdapter<'store>>::Error> +
	Initializable<Error=<Self as PersonAdapter<'store>>::Error> +
	Updatable<Error=<Self as PersonAdapter<'store>>::Error> +
{
	type Error : From<super::Error> + Error;

	/// # Summary
	///
	/// Create a new [`Person`] on the active [`Store`](crate::Store).
	///
	/// # Parameters
	///
	/// See [`Person`].
	///
	/// # Returns
	///
	/// The newly created [`Person`].
	fn create(name: &str, store: &'store Store) -> Result<Person, <Self as PersonAdapter<'store>>::Error>;

	/// # Summary
	///
	/// Retrieve some [`Person`] from the active [`Store`](crate::Store).
	///
	/// # Parameters
	///
	/// See [`Person`].
	///
	/// # Returns
	///
	/// * An `Error`, if something goes wrong.
	/// * A list of matching [`Job`]s.
	fn retrieve(
		query: query::Person,
		store: &Store,
	) -> Result<Vec<Person>, <Self as PersonAdapter<'store>>::Error>;
}
