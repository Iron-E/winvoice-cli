use core::fmt::Display;

use clinvoice_adapter::{
	fmt::{sql, QueryBuilderExt, SnakeCase, TableToSql},
	schema::columns::LocationColumns,
	WriteWhereClause,
};
use clinvoice_match::{Match, MatchLocation, MatchOption, MatchOuterLocation};
use clinvoice_schema::{Id, Location};
use futures::{future, TryFutureExt, TryStreamExt};
use sqlx::{Error, Executor, Postgres, QueryBuilder, Result, Row};

use crate::{fmt::PgLocationRecursiveCte, PgSchema};

mod deletable;
mod location_adapter;
mod updatable;

const COLUMNS: LocationColumns<&str> = LocationColumns::default();

pub struct PgLocation;

impl PgLocation
{
	/// Generate a `WITH RECURSIVE` statement given some `match_condition`.
	///
	///
	/// Contains a `location` identifier, plus a `location_outer` (plus `location_outer_outer`) for
	/// each `match_condition.outer` (`match_condition.outer.outer`, etc.) as well as a
	/// `location_report` which contains all rows of the `locations` table which match the
	/// `match_condition`.
	///
	/// # See also
	///
	/// * [`PgLocationRecursiveCte`] for more about the identifiers.
	pub(super) fn query_with_recursive(match_condition: &MatchLocation) -> QueryBuilder<Postgres>
	{
		/// Generate one expression in a recursive CTE.
		fn generate_expression<T, TOuter>(
			query: &mut QueryBuilder<Postgres>,
			ident: PgLocationRecursiveCte<T, TOuter>,
			match_condition: &MatchLocation,
		) where
			T: Display,
			TOuter: Display,
		{
			let alias = LocationColumns::<char>::DEFAULT_ALIAS;
			let columns = COLUMNS.scope(alias);

			let alias_outer = SnakeCase::from((alias, 'O'));
			let outer_columns = COLUMNS.scope(alias_outer);

			query
				.push(&ident)
				.push(sql::AS)
				.push('(')
				.push(sql::SELECT)
				.push_columns(&outer_columns)
				.push_from(LocationColumns::<&str>::TABLE_NAME, alias_outer);

			if let Some((prev, _)) = ident.slice_end()
			{
				query.push_equijoin(prev, alias, outer_columns.id, columns.outer_id);
			}

			PgSchema::write_where_clause(
				PgSchema::write_where_clause(
					if match_condition.outer == MatchOuterLocation::None
					{
						PgSchema::write_where_clause(
							Default::default(),
							outer_columns.outer_id,
							&MatchOption::<Id>::None,
							query,
						)
					}
					else
					{
						Default::default()
					},
					outer_columns.id,
					&match_condition.id,
					query,
				),
				outer_columns.name,
				&match_condition.name,
				query,
			);

			query.push(')');

			match match_condition.outer
			{
				MatchOuterLocation::Some(ref outer) =>
				{
					query.push(',');
					generate_expression(
						query,
						// HACK: remove `.to_string()` after rust-lang/rust#39959
						PgLocationRecursiveCte::outer(ident.to_string()),
						outer,
					)
				},
				_ =>
				{
					if ident.slice_end().is_some()
					{
						const IDENT_REPORT: PgLocationRecursiveCte<&str, &str> =
							PgLocationRecursiveCte::report();

						query
							.push(',')
							.push(IDENT_REPORT)
							.push(sql::AS)
							.push('(')
							.push(sql::SELECT)
							.push_columns(&columns)
							.push_default_from::<LocationColumns<char>>()
							.push_equijoin(ident, alias_outer, columns.outer_id, outer_columns.id)
							.push(sql::UNION)
							.push(sql::SELECT)
							.push_columns(&columns)
							.push_default_from::<LocationColumns<char>>()
							.push_equijoin(
								IDENT_REPORT,
								alias_outer,
								columns.outer_id,
								outer_columns.id,
							)
							.push(')');
					}
				},
			}
		}

		let mut query = QueryBuilder::new(sql::WITH_RECURSIVE);

		generate_expression(&mut query, PgLocationRecursiveCte::new(), match_condition);

		query.push(' ');
		query
	}

	/// Construct a [`Location`], also constructing all outer [`Location`]s, and return it.
	pub(super) async fn retrieve_by_id(
		connection: impl Executor<'_, Database = Postgres>,
		id: Id,
	) -> Result<Location>
	{
		const SOURCE: &str = "this column in `locations` must be non-null";
		sqlx::query!(
			"WITH RECURSIVE location_view AS
			(
				SELECT id, name, outer_id FROM locations WHERE id = $1
				UNION
				SELECT L.id, L.name, L.outer_id FROM locations L JOIN location_view V ON (L.id = V.outer_id)
			) SELECT * FROM location_view ORDER BY id;",
			id,
		)
		.fetch(connection)
		.try_fold(None, |previous: Option<Location>, view| {
			future::ok(Some(Location {
				id: match view.id
				{
					Some(id) => id,
					_ =>
					{
						return future::err(Error::ColumnDecode {
							index: "name".into(),
							source: SOURCE.into(),
						})
					},
				},
				name: match view.name
				{
					Some(n) => n,
					_ =>
					{
						return future::err(Error::ColumnDecode {
							index: "name".into(),
							source: SOURCE.into(),
						})
					},
				},
				outer: previous.map(Box::new),
			}))
		})
		.await
		.and_then(|v| v.ok_or(Error::RowNotFound))
	}

	/// Retrieve a [`Match`] which will match all of the [`Id`]s of the [`Location`]s which match the
	/// `match_condition`.
	pub(super) async fn retrieve_matching_ids(
		connection: impl Executor<'_, Database = Postgres>,
		match_condition: &MatchLocation,
	) -> Result<Match<Id>>
	{
		let mut query = Self::query_with_recursive(match_condition);

		query
			.push(sql::SELECT)
			.push(COLUMNS.default_scope().id)
			.push_from(
				PgLocationRecursiveCte::from(match_condition),
				LocationColumns::<char>::DEFAULT_ALIAS,
			)
			.prepare()
			.fetch(connection)
			.map_ok(|row| row.get::<Id, _>(COLUMNS.id).into())
			.try_collect()
			.map_ok(Match::Or)
			.await
	}
}
