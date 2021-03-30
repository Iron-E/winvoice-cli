use
{
	crate::{DynResult, io::input},
	clinvoice_adapter::
	{
		Adapters, Error as AdapterError,
		data::{Error as DataError, PersonAdapter, Match},
		Store,
	},
	clinvoice_data::views::PersonView,
};

#[cfg(feature="bincode")]
use clinvoice_adapter_bincode::data::{BincodePerson, Result as BincodeResult};

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
pub(super) fn retrieve_or_err<'store, P>(store: &'store Store) -> DynResult<'store, Vec<PersonView>> where
	P : PersonAdapter<'store> + 'store,
{
	let people = P::retrieve(Match::Any, Match::Any, store)?;

	if people.is_empty()
	{
		return Err(DataError::NoData {entity: stringify!(Person)}.into());
	}

	people.into_iter().try_fold(Vec::new(),
		|mut v, p| -> DynResult<Vec<PersonView>>
		{
			v.push(match store.adapter
			{
				#[cfg(feature="bincode")]
				Adapters::Bincode =>
				{
					let result: BincodeResult<PersonView> = BincodePerson {person: &p, store}.into();
					result
				},

				_ => return Err(AdapterError::FeatureNotFound {adapter: store.adapter}.into()),
			}?);

			Ok(v)
		},
	)
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
pub fn select_one<'store, P, S>(prompt: S, store: &'store Store) -> DynResult<'store, PersonView> where
	P : PersonAdapter<'store> + 'store,
	S : Into<String>,
{
	input::select_one(&retrieve_or_err::<P>(store)?, prompt).map_err(|e| e.into())
}

