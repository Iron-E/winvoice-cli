use
{
	super::{Deletable, Initializable, MatchWhen, Updatable},
	crate::{DynamicResult, Store},
	clinvoice_data::{Contact, Person, Id, views::PersonView},
};

pub trait PersonAdapter<'pass, 'path, 'user> :
	Deletable +
	Initializable<'pass, 'path, 'user> +
	Into<Person> +
	Into<DynamicResult<PersonView>> +
	Into<Store<'pass, 'path, 'user>> +
	Updatable +
{
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
	fn create<'name>(
		contact_info: Vec<Contact>,
		name: &'name str,
		store: Store<'pass, 'path, 'user>,
	) -> DynamicResult<Self>;

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
	) -> DynamicResult<Vec<Self>>;
}
