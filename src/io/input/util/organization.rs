use
{
	crate::{DynResult, io::input},
	clinvoice_adapter::
	{
		data::{Error as DataError, LocationAdapter, OrganizationAdapter},
		Store,
	},
	clinvoice_data::views::OrganizationView,
};

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
pub(super) fn retrieve_or_err<'store, L, O>(store: &'store Store) -> DynResult<'store, Vec<OrganizationView>> where
	L : LocationAdapter<'store> + 'store,
	O : OrganizationAdapter<'store> + 'store,
{
	let organizations = O::retrieve(Default::default(), store)?;

	if organizations.is_empty()
	{
		return Err(DataError::NoData {entity: stringify!(Organization)}.into());
	}

	let organizations_len = organizations.len();
	organizations.into_iter().try_fold(
		Vec::with_capacity(organizations_len),
		|mut v, o| -> DynResult<'store, Vec<OrganizationView>>
		{
			v.push(O::into_view::<L>(o, store)?);

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
pub fn select_one<'store, L, O, S>(prompt: S, store: &'store Store) -> DynResult<'store, OrganizationView> where
	L : LocationAdapter<'store> + 'store,
	O : OrganizationAdapter<'store> + 'store,
	S : Into<String>,
{
	input::select_one(&retrieve_or_err::<L, O>(store)?, prompt).map_err(|e| e.into())
}

