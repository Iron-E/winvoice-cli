use
{
	super::{Deletable, Initializable, MatchWhen, Updatable},
	crate::Store,
	clinvoice_data::{Contact, Person, Id, views::PersonView},
	std::error::Error,
};

pub trait PersonAdapter<'store> :
	Deletable<Error=<Self as PersonAdapter<'store>>::Error> +
	Initializable<Error=<Self as PersonAdapter<'store>>::Error> +
	Into<Person> +
	Into<Result<PersonView, <Self as PersonAdapter<'store>>::Error>> +
	Updatable<Error=<Self as PersonAdapter<'store>>::Error> +
{
	type Error : Error;

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
	fn create(contact_info: Vec<Contact>, name: &str, store: &'store Store) -> Result<Person, <Self as PersonAdapter<'store>>::Error>;

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
		contact_info: MatchWhen<Contact>,
		id: MatchWhen<Id>,
		name: MatchWhen<String>,
		store: &'store Store,
	) -> Result<Vec<Person>, <Self as PersonAdapter<'store>>::Error>;
}
