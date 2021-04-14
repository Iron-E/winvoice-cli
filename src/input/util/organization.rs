use
{
	crate::{DynResult, input},
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
pub(super) fn retrieve_or_err<'err, L, O>(store: &Store) -> DynResult<'err, Vec<OrganizationView>> where
	L : LocationAdapter,
	<L as LocationAdapter>::Error : 'err,
	O : OrganizationAdapter,
	<O as OrganizationAdapter>::Error : 'err,
{
	let organizations = O::retrieve(Default::default(), store)?;

	if organizations.is_empty()
	{
		return Err(DataError::NoData(stringify!(Organization)).into());
	}

	let organizations_len = organizations.len();
	organizations.into_iter().try_fold(
		Vec::with_capacity(organizations_len),
		|mut v, o| -> DynResult<'err, Vec<OrganizationView>>
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
pub fn select_one<'err, L, O, S>(prompt: S, store: &Store) -> DynResult<'err, OrganizationView> where
	L : LocationAdapter,
	<L as LocationAdapter>::Error : 'err,
	O : OrganizationAdapter,
	<O as OrganizationAdapter>::Error : 'err,
	S : Into<String>,
{
	let retrieved = retrieve_or_err::<L, O>(store)?;
	input::select_one(&retrieved, prompt).map_err(|e| e.into())
}

