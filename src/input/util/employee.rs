use
{
	core::fmt::Display,
	std::borrow::Cow::Owned,

	super::menu,
	crate::{app::QUERY_PROMPT, DynResult, filter_map_view, input},

	clinvoice_adapter::
	{
		data::{Error as DataError, EmployeeAdapter, LocationAdapter, OrganizationAdapter, PersonAdapter},
		Store,
	},
	clinvoice_data::{Id, views::EmployeeView},
	clinvoice_query as query,
};

/// # Summary
///
/// Retrieve all [`Employee`][location]s from the specified `store`. If no
/// [`Employee`][location]s are retrieved, return an [error](DataError::NoData).
///
/// # Errors
///
/// * If the [retrieval][L_retrieve] operation fails, its error is forwarded.
/// * If no [`Employee`][location]s are [retrieved][L_retrieve], an [`Error::NoData`] is returned.
/// * If the [selection](input::select) operation fails, its error is forwarded.
///
/// [L_retrieve]: clinvoice_adapter::data::EmployeeAdapter::retrieve
/// [location]: clinvoice_data::Employee
pub fn retrieve_views<'err, D, E, L, O, P>(default_id: Option<Id>, prompt: D, retry_on_empty: bool, store: &Store)
	-> DynResult<'err, Vec<EmployeeView>>
where
	D : Display,
	E : EmployeeAdapter,
	L : LocationAdapter,
	O : OrganizationAdapter,
	P : PersonAdapter,

	<E as EmployeeAdapter>::Error : 'err +
		From<<L as LocationAdapter>::Error> +
		From<<O as OrganizationAdapter>::Error> +
		From<<P as PersonAdapter>::Error>,
	<L as LocationAdapter>::Error : 'err,
	<O as OrganizationAdapter>::Error : 'err,
	<P as PersonAdapter>::Error : 'err,
{
	let query = match default_id
	{
		Some(id) => query::Employee
		{
			id: query::Match::EqualTo(Owned(id)),
			..Default::default()
		},
		_ => input::edit_default(format!("{}\n{}employees", prompt, QUERY_PROMPT))?,
	};

	let results = E::retrieve(&query, &store)?;
	let results_view: Result<Vec<_>, _> = results.into_iter().map(|e|
		E::into_view::<L, O, P>(e, &store)
	).filter_map(|result| filter_map_view!(query, result)).collect();

	if retry_on_empty && results_view.as_ref().map(|r| r.is_empty()).unwrap_or(false) && menu::retry_query()?
	{
		return retrieve_views::<D, E, L, O, P>(default_id, prompt, true, store);
	}

	results_view.map_err(|e| e.into())
}
