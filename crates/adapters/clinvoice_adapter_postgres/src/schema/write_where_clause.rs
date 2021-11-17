use std::{
	borrow::Cow,
	fmt::{Display, Write},
	ops::Deref,
};

use clinvoice_adapter::{WriteContext, WriteJoinClause, WriteWhereClause};
use clinvoice_match::{Match, MatchLocation, MatchOuterLocation, MatchPerson, MatchStr};

use super::PostgresSchema;

const COLUMN_ID: &str = "id";
const COLUMN_NAME: &str = "name";
const COLUMN_OUTER_ID: &str = "outer_id";

/// # Summary
///
/// Write multiple `AND`/`OR` `match_conditions`.
///
/// * If `union` is `true`, the `match_conditions` are separated by `AND`:
///   `[Match::EqualTo(3), Match::LessThan(4)]` is interpreted as `(foo = 3 AND foo < 4)`.
/// * If `union` is `false`, the `match_conditions` are separated by `OR`:
///   `[Match::EqualTo(3), Match::LessThan(4)]` is interpreted as `(foo = 3 OR foo < 4)`.
///
/// NOTE: the above is not proper [`Match`] syntax, since they need to wrap their inner value in a
///       [`std::borrow::Cow`]. View the linked documentation for proper examples.
///
/// The rest of the args are the same as [`WriteSql::write_where`].
fn write_boolean_group<C, I, Q, const UNION: bool>(
	query: &mut String,
	context: C,
	alias: &str,
	match_conditions: &mut I,
) where
	C: Into<WriteContext>,
	I: Iterator<Item = Q>,
	PostgresSchema: WriteWhereClause<Q>,
{
	write!(query, "{} (", context.into().get_prefix()).unwrap();
	if let Some(m) = match_conditions.next()
	{
		PostgresSchema::write_where_clause(WriteContext::InsideClause, alias, m, query);
	}

	let separator: &str = if UNION { " AND" } else { " OR" };
	match_conditions.for_each(|q| {
		query.push_str(separator);
		PostgresSchema::write_where_clause(WriteContext::InsideClause, alias, q, query);
	});

	query.push(')');
}

/// # Summary
///
/// Write a comparison of `alias` and `comparand` using `comparator`.
///
/// The rest of the args are the same as [`WriteSql::write_where`].
fn write_comparison<C>(
	query: &mut String,
	context: C,
	alias: &str,
	comperator: &str,
	comparand: impl Display,
) where
	C: Into<WriteContext>,
{
	write!(
		query,
		"{} {} {} {}",
		context.into().get_prefix(),
		alias,
		comperator,
		comparand
	)
	.unwrap()
}

/// # Summary
///
/// Check if some `alias` has `ANY` or `ALL` of the `values` provided.
///
/// * If `union` is `true`, a check is done to see if `ALL` of the `values` are `alias`.
/// * If `union` is `false`, a check is done to see if `ANY` of the `values` are `alias`.
///
/// NOTE: the above is not proper [`Match`] syntax, since they need to wrap their inner value in a
///       [`std::borrow::Cow`]. View the linked documentation for proper examples.
///
/// The rest of the args are the same as [`WriteSql::write_where`].
fn write_has<'t, C, T>(
	query: &mut String,
	context: C,
	alias: &str,
	values: impl IntoIterator<Item = &'t T>,
	union: bool,
) where
	C: Into<WriteContext>,
	T: 't + Display,
{
	let mut iter = values.into_iter();

	if let Some(id) = iter.next()
	{
		write!(
			query,
			"{} {} {}{}",
			context.into().get_prefix(),
			alias,
			if union { "= ALL(ARRAY[" } else { "IN (" },
			id
		)
		.unwrap()
	}

	iter.for_each(|id| write!(query, ", {}", id).unwrap());

	if union
	{
		query.push_str("])")
	}
	else
	{
		query.push(')')
	}
}

/// # Summary
///
/// Write a comparison of `alias` and `comparand` using `comparator`.
///
/// The rest of the args are the same as [`WriteSql::write_where`].
fn write_is_null<C>(query: &mut String, context: C, alias: &str)
where
	C: Into<WriteContext>,
{
	write!(query, "{} {} IS NULL", context.into().get_prefix(), alias).unwrap()
}

/// # Summary
///
/// Wrap some `match_condition` in `NOT (…)`.
///
/// The args are the same as [`WriteSql::write_where`].
fn write_negated<C, Q>(query: &mut String, context: C, alias: &str, match_condition: Q)
where
	C: Into<WriteContext>,
	PostgresSchema: WriteWhereClause<Q>,
{
	write!(query, "{} NOT (", context.into().get_prefix()).unwrap();
	PostgresSchema::write_where_clause(WriteContext::InsideClause, alias, match_condition, query);
	query.push(')');
}

impl WriteWhereClause<&Match<'_, i64>> for PostgresSchema
{
	fn write_where_clause<C>(
		context: C,
		alias: &str,
		match_condition: &Match<'_, i64>,
		query: &mut String,
	) -> bool
	where
		C: Into<WriteContext>,
	{
		match match_condition
		{
			Match::AllGreaterThan(id) | Match::GreaterThan(id) =>
			{
				write_comparison(query, context, alias, ">", id)
			},
			Match::AllLessThan(id) | Match::LessThan(id) =>
			{
				write_comparison(query, context, alias, "<", id)
			},
			Match::AllInRange(low, high) | Match::InRange(low, high) =>
			{
				write_comparison(query, context, alias, ">=", low);
				write_comparison(query, WriteContext::AfterClause, alias, "<", high);
			},
			Match::And(match_conditions) => write_boolean_group::<_, _, _, true>(
				query,
				context,
				alias,
				&mut match_conditions.iter().filter(|m| *m != &Match::Any),
			),
			Match::Any => return false,
			Match::EqualTo(id) => write_comparison(query, context, alias, "=", id),
			Match::HasAll(ids) => write_has(query, context, alias, ids, true),
			Match::HasAny(ids) => write_has(query, context, alias, ids, false),
			Match::Not(match_condition) => match match_condition.deref()
			{
				Match::Any => write_is_null(query, context, alias),
				m @ _ => write_negated(query, context, alias, m),
			},
			Match::Or(match_conditions) => write_boolean_group::<_, _, _, false>(
				query,
				context,
				alias,
				&mut match_conditions.iter().filter(|m| *m != &Match::Any),
			),
		};
		true
	}
}

impl WriteWhereClause<&MatchStr<Cow<'_, str>>> for PostgresSchema
{
	fn write_where_clause<C>(
		context: C,
		alias: &str,
		match_condition: &MatchStr<Cow<'_, str>>,
		query: &mut String,
	) -> bool
	where
		C: Into<WriteContext>,
	{
		match match_condition
		{
			MatchStr::And(match_conditions) => write_boolean_group::<_, _, _, true>(
				query,
				context,
				alias,
				&mut match_conditions.iter().filter(|m| *m != &MatchStr::Any),
			),
			MatchStr::Any => return false,
			MatchStr::Contains(string) => write!(
				query,
				"{} {} LIKE '%{}%'",
				context.into().get_prefix(),
				alias,
				string
			)
			.unwrap(),
			MatchStr::EqualTo(string) => write!(
				query,
				"{} {} = '{}'",
				context.into().get_prefix(),
				alias,
				string
			)
			.unwrap(),
			MatchStr::Not(match_condition) => match match_condition.deref()
			{
				MatchStr::Any => write_is_null(query, context, alias),
				m @ _ => write_negated(query, context, alias, m),
			},
			MatchStr::Or(match_conditions) => write_boolean_group::<_, _, _, false>(
				query,
				context,
				alias,
				&mut match_conditions.iter().filter(|m| *m != &MatchStr::Any),
			),
			MatchStr::Regex(regex) => write_comparison(query, context, alias, "~", regex),
		};
		true
	}
}

impl WriteWhereClause<&MatchPerson<'_>> for PostgresSchema
{
	/// # Summary
	///
	/// Write a `WHERE` clause for a [`Person`](clinvoice_schema::Person) based on a
	/// `match_condition`. An `alias` can be set to assist in a previous join. The result will be
	/// appended to a `query`.
	///
	/// If the `WHERE` clause has already been started, then pass `keyword_written` as `true`.
	/// Otherwise, if this is the first term in the `WHERE` clause, pass it as `false`.
	fn write_where_clause<C>(
		context: C,
		alias: &str,
		match_condition: &MatchPerson,
		query: &mut String,
	) -> bool
	where
		C: Into<WriteContext>,
	{
		macro_rules! write_where_clause {
			($context:expr, $column:ident, $match_field:ident) => {
				if alias.is_empty()
				{
					PostgresSchema::write_where_clause(
						$context,
						$column,
						&match_condition.$match_field,
						query,
					)
				}
				else
				{
					PostgresSchema::write_where_clause(
						$context,
						&format!("{}.{}", alias, $column),
						&match_condition.$match_field,
						query,
					)
				}
			};
		}

		write_where_clause!(
			write_where_clause!(context, COLUMN_ID, id),
			COLUMN_NAME,
			name
		)
	}
}

impl WriteWhereClause<&MatchLocation<'_>> for PostgresSchema
{
	/// # Summary
	///
	/// Write a `WHERE` clause for a [`Location`](clinvoice_schema::Location) based on a
	/// `match_condition`. An `alias` can be set to assist in a previous join. The result will be
	/// appended to a `query`.
	///
	/// If the `WHERE` clause has already been started, then pass `keyword_written` as `true`.
	/// Otherwise, if this is the first term in the `WHERE` clause, pass it as `false`.
	fn write_where_clause<C>(
		context: C,
		alias: &str,
		match_condition: &MatchLocation,
		query: &mut String,
	) -> bool
	where
		C: Into<WriteContext>,
	{
		// fn recurse(
		// 	query: &mut String,
		// 	alias: &str,
		// 	mut keyword_written: bool,
		// 	match_condition: &MatchLocation,
		// ) -> bool
		fn recurse<C>(
			query: &mut String,
			context: C,
			alias: &str,
			match_condition: &MatchLocation,
		) -> bool
		where
			C: Into<WriteContext>,
		{
			let base_column_id = format!("{}.{}", alias, COLUMN_ID);
			let ctx = match match_condition.outer
			{
				MatchOuterLocation::Any => false,
				MatchOuterLocation::None =>
				{
					write_is_null(query, context, &format!("{}.{}", alias, COLUMN_OUTER_ID));
					true
				},
				MatchOuterLocation::Some(ref outer) =>
				{
					let new_alias = format!("{}O", alias);
					PostgresSchema::write_join_clause(
						query,
						"",
						"locations",
						&new_alias,
						"outer_id",
						&base_column_id,
					)
					.unwrap();
					recurse(query, context, &new_alias, outer.deref())
				},
			};

			PostgresSchema::write_where_clause(
				PostgresSchema::write_where_clause(ctx, &base_column_id, &match_condition.id, query),
				&format!("{}.{}", alias, COLUMN_NAME),
				&match_condition.name,
				query,
			)

			// SELECT L.id, L.name, L.outer_id
			// FROM locations L
			// JOIN locations O ON (O.id = L.outer_id)
			// JOIN locations OO ON (OO.id = O.outer_id)
			// ;
		}

		recurse(query, context, "L", match_condition)
	}
}

#[cfg(test)]
mod tests
{
	use std::borrow::Cow::{Borrowed, Owned};

	use super::{
		Match,
		MatchLocation,
		MatchOuterLocation,
		MatchPerson,
		MatchStr,
		PostgresSchema as Schema,
		WriteContext::{AfterClause, BeforeClause, InsideClause},
		WriteWhereClause,
	};

	#[test]
	fn write_where_clause()
	{
		let mut query = String::new();
		assert!(Schema::write_where_clause(
			BeforeClause,
			"foo",
			&Match::EqualTo(Owned(18)),
			&mut query,
		));
		assert_eq!(query, String::from(" WHERE foo = 18"));

		query.clear();
		assert!(Schema::write_where_clause(
			InsideClause,
			"bar",
			&Match::And(vec![
				Match::Not(Box::new(Match::InRange(Owned(0), Owned(10)))),
				Match::HasAny(
					vec![Owned(0), Owned(9), Owned(7), Owned(4)]
						.into_iter()
						.collect()
				),
				Match::Or(vec![
					Match::Not(Box::new(Match::Any)),
					Match::GreaterThan(Owned(-1)),
				]),
				Match::Any,
			]),
			&mut query,
		));
		assert_eq!(
			query,
			format!(
				" ( NOT ( bar >= 0 AND bar < 10) AND bar IN ({}) AND ( bar IS NULL OR bar > -1))",
				&query[44..54],
			)
		);

		{
			query.clear();
			let mut query2 = String::new();

			assert!(Schema::write_where_clause(
				InsideClause,
				"another_row",
				&Match::AllInRange(Owned(0), Owned(2)),
				&mut query,
			));
			assert!(Schema::write_where_clause(
				InsideClause,
				"another_row",
				&Match::InRange(Owned(0), Owned(2)),
				&mut query2,
			));
			assert_eq!(query, query2);

			query.clear();
			query2.clear();
			assert!(Schema::write_where_clause(
				InsideClause,
				"another_row",
				&Match::AllLessThan(Owned(0)),
				&mut query,
			));
			assert!(Schema::write_where_clause(
				InsideClause,
				"another_row",
				&Match::LessThan(Owned(0)),
				&mut query2,
			));
			assert_eq!(query, query2);

			query.clear();
			query2.clear();
			assert!(Schema::write_where_clause(
				InsideClause,
				"another_row",
				&Match::AllGreaterThan(Owned(0)),
				&mut query,
			));
			assert!(Schema::write_where_clause(
				InsideClause,
				"another_row",
				&Match::GreaterThan(Owned(0)),
				&mut query2,
			));
			assert_eq!(query, query2);
		}

		query.clear();
		assert!(!Schema::write_where_clause(
			InsideClause,
			"bar",
			&MatchStr::Any,
			&mut query,
		));
		assert_eq!(query, String::from(""));

		query.clear();
		assert!(Schema::write_where_clause(
			AfterClause,
			"bar",
			&MatchStr::Contains(Borrowed("punky brüster")),
			&mut query,
		));
		assert_eq!(query, String::from(" AND bar LIKE '%punky brüster%'"));

		query.clear();
		assert!(Schema::write_where_clause(
			BeforeClause,
			"some_row",
			&MatchStr::Or(vec![
				MatchStr::Regex(Borrowed(r#"^f.rk.*\bit\b.*over$"#)),
				MatchStr::Not(Box::new(MatchStr::EqualTo(Borrowed("not equal")))),
			]),
			&mut query,
		));
		assert_eq!(
			query,
			String::from(
				r#" WHERE ( some_row ~ ^f.rk.*\bit\b.*over$ OR NOT ( some_row = 'not equal'))"#
			),
		);
	}

	#[test]
	fn write_person_where_clause()
	{
		let mut query = String::new();
		Schema::write_where_clause(false, "", &MatchPerson::default(), &mut query);
		assert!(query.is_empty());

		query.clear();
		Schema::write_where_clause(
			false,
			"",
			&MatchPerson {
				id: Match::EqualTo(Owned(7)),
				..Default::default()
			},
			&mut query,
		);
		assert_eq!(query, String::from(" WHERE id = 7"));

		query.clear();
		Schema::write_where_clause(
			true,
			"",
			&MatchPerson {
				id:   Match::EqualTo(Owned(7)),
				name: MatchStr::EqualTo(Borrowed("stuff")),
			},
			&mut query,
		);
		assert_eq!(query, String::from(" AND id = 7 AND name = 'stuff'"),);
	}

	#[test]
	fn write_location_join_where_clause()
	{
		let mut query = String::new();
		Schema::write_where_clause(false, "", &MatchLocation::default(), &mut query);
		assert!(query.is_empty());

		query.clear();
		Schema::write_where_clause(
			false,
			"",
			&MatchLocation {
				id: Match::EqualTo(Owned(7)),
				..Default::default()
			},
			&mut query,
		);
		assert_eq!(query, String::from(" WHERE L.id = 7"),);
	}
}
