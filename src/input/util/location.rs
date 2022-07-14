use core::fmt::Display;

use clinvoice_adapter::{schema::LocationAdapter, Deletable};
use clinvoice_match::MatchLocation;
use clinvoice_schema::Location;
use sqlx::{Database, Executor, Pool};

use super::{menu, MATCH_PROMPT};
use crate::{input, DynResult};

/// # Summary
///
/// Retrieve all [`Location`][location]s from the specified `store`. If no
/// [`Location`][location]s are retrieved, return an [error](DataError::NoData).
///
/// # Errors
///
/// * If the [retrieval][L_retrieve] operation fails, its error is forwarded.
/// * If no [`Location`][location]s are [retrieved][L_retrieve], an [`Error::NoData`] is returned.
/// * If the [selection](input::select) operation fails, its error is forwarded.
///
/// [L_retrieve]: clinvoice_adapter::schema::LocationAdapter::retrieve
/// [location]: clinvoice_schema::Location
pub async fn retrieve<D, Db, LAdapter, const RETRY_ON_EMPTY: bool>(
	connection: &Pool<Db>,
	prompt: D,
) -> DynResult<Vec<Location>>
where
	D: Display,
	Db: Database,
	LAdapter: Deletable<Db = Db> + LocationAdapter,
	for<'c> &'c mut Db::Connection: Executor<'c, Database = Db>,
{
	loop
	{
		let match_condition: MatchLocation =
			input::edit_default(format!("{prompt}\n{MATCH_PROMPT}locations"))?;

		let results = LAdapter::retrieve(connection, &match_condition).await?;

		if RETRY_ON_EMPTY && results.is_empty() && menu::ask_to_retry()?
		{
			continue;
		}

		return Ok(results);
	}
}

/// # Summary
///
/// Retrieve all [`Location`][location]s from the specified `store`. If no
/// [`Location`][location]s are retrieved, return an [error](DataError::NoData).
///
/// # Errors
///
/// * If the [retrieval][L_retrieve] operation fails, its error is forwarded.
/// * If no [`Location`][location]s are [retrieved][L_retrieve], an [`Error::NoData`] is returned.
/// * If the [selection](input::select) operation fails, its error is forwarded.
///
/// [L_retrieve]: clinvoice_adapter::schema::LocationAdapter::retrieve
/// [location]: clinvoice_schema::Location
pub async fn select_one<D, Db, LAdapter, const RETRY_ON_EMPTY: bool>(
	connection: &Pool<Db>,
	prompt: D,
) -> DynResult<Location>
where
	D: Display,
	Db: Database,
	LAdapter: Deletable<Db = Db> + LocationAdapter,
	for<'c> &'c mut Db::Connection: Executor<'c, Database = Db>,
{
	let locations = retrieve::<D, Db, LAdapter, RETRY_ON_EMPTY>(connection, prompt).await?;

	let location = input::select_one(&locations, "Select the `Location`")?;

	Ok(location)
}
