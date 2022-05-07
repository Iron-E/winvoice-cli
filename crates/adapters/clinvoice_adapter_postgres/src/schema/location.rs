use core::fmt::Write;

use clinvoice_adapter::WriteWhereClause;
use clinvoice_match::{Match, MatchLocation, MatchOuterLocation};
use clinvoice_schema::{Id, Location};
use futures::{future, TryFutureExt, TryStreamExt};
use sqlx::{Executor, Postgres, Result, Row};

use crate::PgSchema as Schema;

mod deletable;
mod location_adapter;
mod updatable;

pub struct PgLocation;

impl PgLocation
{
	pub(super) async fn retrieve_matching_ids<'a>(
		connection: impl Executor<'_, Database = Postgres>,
		match_condition: &MatchLocation,
	) -> Result<Match<Id>>
	{
		struct Cte<'a>
		{
			current: &'a str,
			previous: &'a str,
		}

		/// # Summary
		///
		/// Generate multiple Common Table Expressions for a recursive query.
		fn generate_cte(first: bool, query: &mut String, cte: Cte, match_condition: &MatchLocation)
		{
			writeln!(
				query,
				" {} as ( SELECT LO.id, LO.name, LO.outer_id FROM locations LO {}JOIN {} L ON (LO.id \
				 = L.outer_id)",
				cte.current,
				if cte.previous.is_empty() { "-- " } else { "" },
				cte.previous,
			)
			.unwrap();
			Schema::write_where_clause(
				Schema::write_where_clause(
					if match_condition.outer == MatchOuterLocation::None
					{
						Schema::write_where_clause(
							Default::default(),
							"LO.outer_id",
							&Match::Not(Match::<i64>::Always.into()),
							query,
						)
					}
					else
					{
						Default::default()
					},
					"LO.id",
					&match_condition.id,
					query,
				),
				"LO.name",
				&match_condition.name,
				query,
			);
			write!(query, "),").unwrap();

			match match_condition.outer
			{
				MatchOuterLocation::Always | MatchOuterLocation::None =>
				{
					if first
					{
						query.pop();
					}
					else
					{
						write!(
							query,
							" location_report AS ( SELECT L.id, L.name, L.outer_id FROM locations L JOIN \
							 {} LO ON (L.outer_id = LO.id) UNION SELECT L.id, L.name, L.outer_id FROM \
							 locations L JOIN location_report LO ON (L.outer_id = LO.id))",
							cte.current,
						)
						.unwrap()
					}
					write!(
						query,
						" SELECT id FROM {};",
						if first
						{
							cte.current
						}
						else
						{
							"location_report"
						},
					)
					.unwrap()
				},
				MatchOuterLocation::Some(ref outer) => generate_cte(
					false,
					query,
					Cte {
						current: &format!("{}_outer", cte.current),
						previous: cte.current,
					},
					outer,
				),
			}
		}

		let mut query = String::from("WITH RECURSIVE");
		generate_cte(
			true,
			&mut query,
			Cte {
				current: "location",
				previous: "",
			},
			match_condition,
		);
		Ok(Match::HasAny(
			sqlx::query(&query)
				.fetch(connection)
				.map_ok(|row| row.get::<Id, _>("id"))
				.try_collect()
				.await?,
		))
	}

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
			future::ok(Some(Location {
				id: view.id.expect("`locations` table should have non-null ID"),
				name: view
					.name
					.expect("`locations` table should have non-null name"),
				outer: previous.map(Box::new),
			}))
		})
		.map_ok(|v| v.expect("A database object failed to be returned by recursive query"))
		.await
	}
}
