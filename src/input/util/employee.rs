use std::{borrow::Cow::Owned, fmt::Display};

use clinvoice_adapter::data::{Deletable, EmployeeAdapter};
use clinvoice_data::{views::EmployeeView, Id};
use clinvoice_query as query;
use sqlx::{Database, Pool};

use super::menu;
use crate::{app::QUERY_PROMPT, input, DynResult};

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
pub async fn retrieve_view<'err, D, Db, EAdapter>(
	connection: &Pool<Db>,
	default_id: Option<Id>,
	prompt: D,
	retry_on_empty: bool,
) -> DynResult<'err, Vec<EmployeeView>>
where
	D: Display,
	Db: Database,
	EAdapter: Deletable<Db = Db> + EmployeeAdapter + Send,
{
	loop
	{
		let query = match default_id
		{
			Some(id) => query::Employee {
				id: query::Match::EqualTo(Owned(id)),
				..Default::default()
			},
			_ => input::edit_default(format!("{}\n{}employees", prompt, QUERY_PROMPT))?,
		};

		let results = EAdapter::retrieve_view(connection, &query).await?;

		if retry_on_empty &&
			results.is_empty() &&
			menu::retry_query()?
		{
			continue;
		}

		return Ok(results);
	}
}
