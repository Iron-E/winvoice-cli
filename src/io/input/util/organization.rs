use
{
	crate::{DynResult, io::input},
	clinvoice_adapter::
	{
		Adapters, Error as AdapterError,
		data::{Error as DataError, OrganizationAdapter, MatchWhen},
		Store,
	},
	clinvoice_data::views::OrganizationView,
};

#[cfg(feature="bincode")]
use clinvoice_adapter_bincode::data::{BincodeOrganization, Result as BincodeResult};

/// # Summary
///
/// Retrieve all [`Organization`][organization]s from the specified `store`. If no
/// [`Organization`][organization]s are retrieved, return an [error](DataError::NoData).
///
/// # Errors
///
/// * If the [retrieval][P_retrieve] operation fails, its error is forwarded.
/// * If no [`Organization`][organization]s are [retrieved][P_retrieve], an [`Error::NoData`] is returned.
/// * If the [selection](input::select) operation fails, its error is forwarded.
///
/// [P_retrieve]: clinvoice_adapter::data::OrganizationAdapter::retrieve
/// [organization]: clinvoice_data::Organization
pub(super) fn retrieve_or_err<'store, O>(store: &'store Store) -> DynResult<'store, Vec<OrganizationView>> where
	O : OrganizationAdapter<'store> + 'store,
{
	let organizations = O::retrieve(MatchWhen::Any, MatchWhen::Any, MatchWhen::Any, store)?;

	if organizations.is_empty()
	{
		return Err(DataError::NoData {entity: stringify!(Organization)}.into());
	}

	organizations.into_iter().try_fold(Vec::new(),
		|mut v, o| -> DynResult<Vec<OrganizationView>>
		{
			v.push(match store.adapter
			{
				#[cfg(feature="bincode")]
				Adapters::Bincode =>
				{
					let result: BincodeResult<OrganizationView> = BincodeOrganization {organization: &o, store}.into();
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
/// `prompt` the user to [select](input::select) one [`Location`][organization] from the specified `store`.
///
/// # Errors
///
/// * If [`retrieve_or_err`] fails.
/// * If [`input::select_one`] fails.
///
/// [organization]: clinvoice_data::Organization
pub fn select_one<'store, P, S>(prompt: S, store: &'store Store) -> DynResult<'store, OrganizationView> where
	P : OrganizationAdapter<'store> + 'store,
	S : Into<String>,
{
	input::select_one(&retrieve_or_err::<P>(store)?, prompt).map_err(|e| e.into())
}

