use
{
	clinvoice_adapter::
	{
		data::{Error as DataError, PersonAdapter},
		Store,
	},
	clinvoice_data::views::PersonView,
};

/// # Summary
///
/// Retrieve all [`Person`][person]s from the specified `store`. If no
/// [`Person`][person]s are retrieved, return an [error](DataError::NoData).
///
/// # Errors
///
/// * If the [retrieval][P_retrieve] operation fails, its error is forwarded.
/// * If no [`Person`][person]s are [retrieved][P_retrieve], an [`Error::NoData`] is returned.
/// * If the [selection](input::select) operation fails, its error is forwarded.
///
/// [P_retrieve]: clinvoice_adapter::data::PersonAdapter::retrieve
/// [person]: clinvoice_data::Person
pub fn retrieve_views<P>(store: &Store) -> Result<Vec<PersonView>, <P as PersonAdapter>::Error> where
	P : PersonAdapter,
{
	let people = P::retrieve(Default::default(), store)?;

	if people.is_empty()
	{
		return Err(DataError::NoData(stringify!(Person)).into());
	}

	Ok(people.into_iter().map(|p| p.into()).collect())
}
