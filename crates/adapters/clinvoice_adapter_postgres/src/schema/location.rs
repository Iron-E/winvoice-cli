use core::fmt::Display;

use clinvoice_adapter::{
	fmt::{sql, QueryBuilderExt, SnakeCase, TableToSql},
	schema::columns::LocationColumns,
	WriteWhereClause,
};
use clinvoice_match::{Match, MatchLocation, MatchOuterLocation};
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
	/// # Summary
	///
	/// Generate a `WITH RECURSIVE` statement which contains a `location` for `match_condition`, and
	/// `location_outer` (plus `location_outer_outer`) for each `match_condition.outer` (plus
	/// `match_condition.outer.outer`) as well as a `location_report` which contains all rows of the
	/// `locations` table which match the `match_condition`.
	pub(super) fn query_with_recursive(match_condition: &MatchLocation) -> QueryBuilder<Postgres>
	{
		/// # Summary
		///
		/// Generate multiple Common Table Expressions for a recursive query.
		fn generate_cte<TCurrent, TPrev>(
			query: &mut QueryBuilder<Postgres>,
			ident: SnakeCase<TPrev, TCurrent>,
			match_condition: &MatchLocation,
		) where
			TCurrent: Display,
			TPrev: Display,
		{
			let alias = LocationColumns::<char>::default_alias();
			let columns = COLUMNS.scope(alias);

			let alias_outer = SnakeCase::from((alias, 'O'));
			let outer_columns = COLUMNS.scope(alias_outer);

			query
				.push(&ident)
				.push(sql::AS)
				.push('(')
				.push(sql::SELECT)
				.push_columns(&outer_columns)
				.push_from(LocationColumns::<&str>::table_name(), alias_outer);

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
							&Match::Not(Match::<i64>::Any.into()),
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
					generate_cte(
						query,
						// HACK: remove `.to_string()` after rust-lang/rust#39959
						PgLocationRecursiveCte::outer(ident.to_string()),
						outer,
					)
				},
				_ =>
				{
					if let Some(_) = ident.slice_end()
					{
						const IDENT_REPORT: SnakeCase<&str, &str> = PgLocationRecursiveCte::report();

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

		generate_cte(&mut query, PgLocationRecursiveCte::new(), match_condition);

		query.push(' ');
		query
	}

	/// # Summary
	///
	/// Recursively constructs a [`Location`] which matches the `id` in the database over the `connection`.
	///
	/// # Panics
	///
	/// If `id` does not match any row in the database.
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
		.map_ok(|v| v.expect("`id` did not match any row in the database"))
		.await
	}

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
				LocationColumns::<char>::default_alias(),
			)
			.prepare()
			.fetch(connection)
			.map_ok(|row| row.get::<Id, _>(COLUMNS.id).into())
			.try_collect()
			.map_ok(Match::Or)
			.await
	}
}
