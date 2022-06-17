use core::fmt::Display;

use clinvoice_adapter::{schema::columns::LocationColumns, WriteWhereClause};
use clinvoice_match::{Match, MatchLocation, MatchOuterLocation};
use clinvoice_schema::{Id, Location};
use futures::{future, TryFutureExt, TryStreamExt};
use sqlx::{Error, Executor, Postgres, QueryBuilder, Result, Row};

use crate::{fmt::PgLocationRecursiveCte, PgSchema};

mod deletable;
mod location_adapter;
mod updatable;

const COLUMNS: LocationColumns<&'static str> = LocationColumns::default();
const ALIAS_INNER: &str = "L";
const ALIAS_OUTER: &str = "LO";

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
		fn generate_cte<TCurrent, TInner, const FIRST: bool>(
			query: &mut QueryBuilder<Postgres>,
			ident: PgLocationRecursiveCte<TCurrent, TInner>,
			match_condition: &MatchLocation,
		) where
			TCurrent: Display,
			TInner: Display,
		{
			let inner_columns = COLUMNS.scoped(ALIAS_INNER);
			let outer_columns = COLUMNS.scoped(ALIAS_OUTER);

			// NOTE: this scope exists because we want to get rid of the mutable borrow after we're
			//       done with it.
			{
				let mut separated = query.separated(' ');

				separated
					.push(ident)
					.push("AS (SELECT")
					.push(outer_columns.id)
					.push_unseparated(',')
					.push_unseparated(outer_columns.name)
					.push_unseparated(',')
					.push_unseparated(outer_columns.outer_id)
					.push("FROM locations")
					.push(ALIAS_OUTER);

				if let Some(inner) = ident.inner()
				{
					separated
						.push("JOIN")
						.push(inner)
						.push(ALIAS_INNER)
						.push("ON (")
						.push_unseparated(outer_columns.id)
						.push_unseparated('=')
						.push_unseparated(inner_columns.outer_id)
						.push_unseparated(')');
				}
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
					generate_cte::<_, _, false>(query, ident.outer(), outer)
				},
				MatchOuterLocation::Any | MatchOuterLocation::None if !FIRST =>
				{
					query
						.separated(' ')
						.push(", location_report AS (SELECT")
						.push(inner_columns.id)
						.push_unseparated(',')
						.push_unseparated(inner_columns.name)
						.push_unseparated(',')
						.push_unseparated(inner_columns.outer_id)
						.push("FROM locations")
						.push(ALIAS_INNER)
						.push("JOIN")
						.push(ident)
						.push(ALIAS_OUTER)
						.push("ON (")
						.push_unseparated(inner_columns.outer_id)
						.push_unseparated('=')
						.push_unseparated(outer_columns.id)
						.push_unseparated(") UNION SELECT")
						.push(inner_columns.id)
						.push_unseparated(',')
						.push_unseparated(inner_columns.name)
						.push_unseparated(',')
						.push_unseparated(inner_columns.outer_id)
						.push("FROM locations")
						.push(ALIAS_INNER)
						.push("JOIN location_report")
						.push(ALIAS_OUTER)
						.push("ON (")
						.push_unseparated(inner_columns.outer_id)
						.push_unseparated('=')
						.push_unseparated(outer_columns.id)
						.push_unseparated("))");
				},
			}
		}

		let mut query = QueryBuilder::new("WITH RECURSIVE ");

		generate_cte::<_, _, true>(
			&mut query,
			PgLocationRecursiveCte::innermost(),
			match_condition,
		);

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
			let id = match view.id
			{
				Some(id) => id,
				_ =>
				{
					return future::err(Error::ColumnDecode {
						index: "id".into(),
						source: "this column in `locations` must be non-null".into(),
					})
				},
			};

			let name = match view.name
			{
				Some(n) => n,
				_ =>
				{
					return future::err(Error::ColumnDecode {
						index: "name".into(),
						source: "this column in `locations` must be non-null".into(),
					})
				},
			};

			future::ok(Some(Location {
				id,
				name,
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
			.separated(' ')
			.push("SELECT")
			.push(COLUMNS.id)
			.push("FROM")
			.push(
				if match_condition.outer == MatchOuterLocation::None
				{
					PgLocationRecursiveCte::innermost()
				}
				else
				{
					PgLocationRecursiveCte::report()
				},
			);

		query
			.push(';')
			.build()
			.fetch(connection)
			.map_ok(|row| row.get::<Id, _>(COLUMNS.id).into())
			.try_collect()
			.map_ok(Match::Or)
			.await
	}
}
