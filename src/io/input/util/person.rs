use
{
	crate::{DynResult, io::input},

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
pub(super) fn retrieve_or_err<'err, P>(store: &Store) -> DynResult<'err, Vec<PersonView>> where
	P : PersonAdapter,
	<P as PersonAdapter>::Error : 'err,
{
	let people = P::retrieve(Default::default(), store)?;

	if people.is_empty()
	{
		return Err(DataError::NoData(stringify!(Person)).into());
	}

	Ok(people.into_iter().map(|p| p.into()).collect())
}

/// # Summary
///
/// `prompt` the user to [select](input::select) one [`Location`][person] from the specified `store`.
///
/// # Errors
///
/// * If [`retrieve_or_err`] fails.
/// * If [`input::select_one`] fails.
///
/// [person]: clinvoice_data::Person
pub fn select_one<'err, P, S>(prompt: S, store: &Store) -> DynResult<'err, PersonView> where
	P : PersonAdapter,
	<P as PersonAdapter>::Error : 'err,
	S : Into<String>,
{
	let retrieved = retrieve_or_err::<P>(store)?;
	input::select_one(&retrieved, prompt).map_err(|e| e.into())
}
