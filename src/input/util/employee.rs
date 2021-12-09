use std::{borrow::Cow::Owned, fmt::Display};

use clinvoice_adapter::{schema::EmployeeAdapter, Deletable};
use clinvoice_match::{Match, MatchEmployee};
use clinvoice_schema::{views::EmployeeView, Id};
use sqlx::{Database, Executor, Pool};

use super::{menu, MATCH_PROMPT};
use crate::{input, DynResult};

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
/// [L_retrieve]: clinvoice_adapter::schema::EmployeeAdapter::retrieve
/// [location]: clinvoice_schema::Employee
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
	for<'c> &'c mut Db::Connection: Executor<'c, Database = Db>,
{
	loop
	{
		let query = match default_id
		{
			Some(id) => MatchEmployee {
				id: id.into(),
				..Default::default()
			},
			_ => input::edit_default(format!("{}\n{}employees", prompt, MATCH_PROMPT))?,
		};

		let results = EAdapter::retrieve_view(connection, &query).await?;

		if retry_on_empty && results.is_empty() && menu::ask_to_retry()?
		{
			continue;
		}

		return Ok(results);
	}
}
