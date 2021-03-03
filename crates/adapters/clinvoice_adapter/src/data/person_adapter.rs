use
{
	super::{Deletable, Initializable, MatchWhen, Updatable},
	crate::Store,
	clinvoice_data::{Contact, Person, Id, views::PersonView},
	std::error::Error,
};

pub trait PersonAdapter<'pass, 'path, 'user> :
	Deletable<Self::Error> +
	Initializable<Self::Error> +
	Into<Person> +
	Into<Result<PersonView, Self::Error>> +
	Into<Store<'pass, 'path, 'user>> +
	Updatable<Self::Error> +
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
	fn create(
		contact_info: Vec<Contact>,
		name: &str,
		store: Store<'pass, 'path, 'user>,
	) -> Result<Self, Self::Error>;

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
		store: Store<'pass, 'path, 'user>,
	) -> Result<Vec<Self>, Self::Error>;
}
