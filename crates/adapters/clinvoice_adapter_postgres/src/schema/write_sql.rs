use std::{
	fmt::{Debug, Write},
	hash::Hash,
	ops::Deref,
};

use clinvoice_adapter::WriteSql;
use clinvoice_query::{Match, MatchStr};

use super::PostgresSchema;

/// # Summary
///
/// Write multiple `AND`/`OR` `queries`.
///
/// * If `union` is `true`, the `queries` are separated by `AND`:
///   `[Match::EqualTo(3), Match::LessThan(4)]` is interpreted as `(foo = 3 AND foo < 4)`.
/// * If `union` is `false`, the `queries` are separated by `OR`:
///   `[Match::EqualTo(3), Match::LessThan(4)]` is interpreted as `(foo = 3 OR foo < 4)`.
///
/// NOTE: the above is not proper [`Match`] syntax, since they need to wrap their inner value in a
///       [`std::borrow::Cow`]. View the linked documentation for proper examples.
///
/// The rest of the args are the same as [`WriteSql::write_where`].
fn write_boolean_group<Q>(
	column: &'static str,
	prefix: Option<&'static str>,
	queries: &[Q],
	sql: &mut String,
	union: bool,
) where
	PostgresSchema: WriteSql<Q>,
{
	prefix.map(|p| write!(sql, " {}", p).unwrap());
	queries
		.first()
		.map(|q| PostgresSchema::write_where(column, Some("("), q, sql));
	let separator = Some(if union { "AND" } else { "OR" });
	queries.iter().skip(1).for_each(|q| {
		PostgresSchema::write_where(column, separator, q, sql);
	});
	write!(sql, ")").unwrap();
}

/// # Summary
///
/// Write multiple `AND`/`OR` `queries`.
///
/// * If `union` is `true`, the `queries` are separated by `AND`:
///   `[Match::EqualTo(3), Match::LessThan(4)]` is interpreted as `(foo = 3 AND foo < 4)`.
/// * If `union` is `false`, the `queries` are separated by `OR`:
///   `[Match::EqualTo(3), Match::LessThan(4)]` is interpreted as `(foo = 3 OR foo < 4)`.
///
/// NOTE: the above is not proper [`Match`] syntax, since they need to wrap their inner value in a
///       [`std::borrow::Cow`]. View the linked documentation for proper examples.
///
/// The rest of the args are the same as [`WriteSql::write_where`].
fn write_negated<Q>(
	column: &'static str,
	prefix: Option<&'static str>,
	query: &Box<Q>,
	sql: &mut String,
) where
	PostgresSchema: WriteSql<Q>,
{
	prefix.map(|p| write!(sql, " {}", p).unwrap());
	PostgresSchema::write_where(column, Some("NOT ("), query.deref(), sql);
	write!(sql, ") ").unwrap();
}


impl WriteSql<Match<'_, i64>> for PostgresSchema
{
	fn write_where(
		column: &'static str,
		prefix: Option<&'static str>,
		query: &Match<'_, i64>,
		sql: &mut String,
	) -> bool
	{
		match query
		{
			Match::AllGreaterThan(id) | Match::GreaterThan(id) =>
			{
				write!(sql, " {} {} > {}", prefix.unwrap_or_default(), column, id).unwrap()
			},
			Match::AllLessThan(id) | Match::LessThan(id) =>
			{
				write!(sql, " {} {} < {}", prefix.unwrap_or_default(), column, id).unwrap()
			},
			Match::AllInRange(low, high) | Match::InRange(low, high) => write!(
				sql,
				" {} {} <= {column} AND {column} < {}",
				prefix.unwrap_or_default(),
				low,
				high,
				column = column,
			)
			.unwrap(),
			Match::And(queries) => write_boolean_group(column, prefix, queries, sql, false),
			Match::Any => return false,
			Match::EqualTo(id) => write!(sql, " {} id = {}", prefix.unwrap_or_default(), id).unwrap(),
			Match::HasAll(ids) =>
			{
				let mut iter = ids.iter();
				iter.next().map(|id| {
					write!(sql, " {} id = ALL(ARRAY[{}", prefix.unwrap_or_default(), id).unwrap()
				});
				iter.for_each(|id| write!(sql, ", {}", id).unwrap());
				write!(sql, "])").unwrap();
			},
			Match::HasAny(ids) =>
			{
				let mut iter = ids.iter();
				iter
					.next()
					.map(|id| write!(sql, " {} id IN ({}", prefix.unwrap_or_default(), id).unwrap());
				iter.for_each(|id| write!(sql, ", {}", id).unwrap());
				write!(sql, ")").unwrap();
			},
			Match::Not(query) => write_negated(column, prefix, query, sql),
			Match::Or(queries) => write_boolean_group(column, prefix, queries, sql, false),
		};
		true
	}
}

impl WriteSql<MatchStr<String>> for PostgresSchema
{
	fn write_where(
		column: &'static str,
		prefix: Option<&'static str>,
		query: &MatchStr<String>,
		sql: &mut String,
	) -> bool
	{
		match query
		{
			MatchStr::And(queries) => write_boolean_group(column, prefix, queries, sql, false),
			MatchStr::Any => return false,
			MatchStr::Contains(string) => write!(
				sql,
				" {} {} LIKE '%{}%'",
				prefix.unwrap_or_default(),
				column,
				string,
			).unwrap(),
			MatchStr::EqualTo(string) => write!(
				sql,
				" {} {} = {}",
				prefix.unwrap_or_default(),
				column,
				string,
			).unwrap(),
			MatchStr::Not(query) => write_negated(column, prefix, query, sql),
			MatchStr::Or(queries) => write_boolean_group(column, prefix, queries, sql, false),
			MatchStr::Regex(regex) => write!(
				sql,
				" {} {} ~ {}",
				prefix.unwrap_or_default(),
				column,
				regex,
			).unwrap(),
		};
		true
	}
}
