use std::{
	fmt::{Display, Write},
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
	prefix: Option<&'static str>,
	column: &'static str,
	queries: &[Q],
	union: bool,
	sql: &mut String,
) where
	PostgresSchema: WriteSql<Q>,
{
	prefix.map(|p| write!(sql, " {}", p).unwrap());
	queries
		.first()
		.map(|q| PostgresSchema::write_where(Some("("), column, q, sql));
	let separator = Some(if union { "AND" } else { "OR" });
	queries.iter().skip(1).for_each(|q| {
		PostgresSchema::write_where(separator, column, q, sql);
	});
	write!(sql, ")").unwrap();
}

/// # Summary
///
/// Write a comparison of `column` and `comparand` using `comparator`.
///
/// The rest of the args are the same as [`WriteSql::write_where`].
fn write_comparison(
	prefix: Option<&'static str>,
	column: &'static str,
	comperator: &'static str,
	comparand: impl Display,
	sql: &mut String,
)
{
	write!(
		sql,
		" {} {} {} {}",
		prefix.unwrap_or_default(),
		column,
		comperator,
		comparand
	)
	.unwrap()
}

/// # Summary
///
/// Check if some `column` has `ANY` or `ALL` of the `values` provided.
///
/// * If `union` is `true`, a check is done to see if `ALL` of the `values` are `column`.
/// * If `union` is `false`, a check is done to see if `ANY` of the `values` are `column`.
///
/// NOTE: the above is not proper [`Match`] syntax, since they need to wrap their inner value in a
///       [`std::borrow::Cow`]. View the linked documentation for proper examples.
///
/// The rest of the args are the same as [`WriteSql::write_where`].
fn write_has<'t, T>(
	prefix: Option<&'static str>,
	column: &'static str,
	values: impl IntoIterator<Item = &'t T>,
	union: bool,
	sql: &mut String,
) where
	T: 't + Display,
{
	let mut iter = values.into_iter();
	iter.next().map(|id| {
		write!(
			sql,
			" {} {} = {}{}",
			if union { "ALL(ARRAY[" } else { "IN (" },
			prefix.unwrap_or_default(),
			column,
			id
		)
		.unwrap()
	});
	iter.for_each(|id| write!(sql, ", {}", id).unwrap());
	if union
	{
		write!(sql, "])")
	}
	else
	{
		write!(sql, ")")
	}
	.unwrap()
}

/// # Summary
///
/// Wrap some `query` in `NOT (…)`.
///
/// The args are the same as [`WriteSql::write_where`].
fn write_negated<Q>(
	prefix: Option<&'static str>,
	column: &'static str,
	query: &Box<Q>,
	sql: &mut String,
) where
	PostgresSchema: WriteSql<Q>,
{
	prefix.map(|p| write!(sql, " {}", p).unwrap());
	PostgresSchema::write_where(Some("NOT ("), column, query.deref(), sql);
	write!(sql, ") ").unwrap();
}

impl WriteSql<Match<'_, i64>> for PostgresSchema
{
	fn write_where(
		prefix: Option<&'static str>,
		column: &'static str,
		query: &Match<'_, i64>,
		sql: &mut String,
	) -> bool
	{
		match query
		{
			Match::AllGreaterThan(id) | Match::GreaterThan(id) =>
			{
				write_comparison(prefix, column, ">", id, sql)
			},
			Match::AllLessThan(id) | Match::LessThan(id) =>
			{
				write_comparison(prefix, column, "<", id, sql)
			},
			Match::AllInRange(low, high) | Match::InRange(low, high) =>
			{
				write_comparison(prefix, column, ">=", low, sql);
				write_comparison(Some("AND"), column, "<", high, sql);
			},
			Match::And(queries) => write_boolean_group(prefix, column, queries, true, sql),
			Match::Any => return false,
			Match::EqualTo(id) => write_comparison(prefix, column, "=", id, sql),
			Match::HasAll(ids) => write_has(prefix, column, ids, true, sql),
			Match::HasAny(ids) => write_has(prefix, column, ids, false, sql),
			Match::Not(query) => write_negated(prefix, column, query, sql),
			Match::Or(queries) => write_boolean_group(prefix, column, queries, false, sql),
		};
		true
	}
}

impl WriteSql<MatchStr<String>> for PostgresSchema
{
	fn write_where(
		prefix: Option<&'static str>,
		column: &'static str,
		query: &MatchStr<String>,
		sql: &mut String,
	) -> bool
	{
		match query
		{
			MatchStr::And(queries) => write_boolean_group(prefix, column, queries, true, sql),
			MatchStr::Any => return false,
			MatchStr::Contains(string) => write!(
				sql,
				" {} {} LIKE '%{}%'",
				prefix.unwrap_or_default(),
				column,
				string,
			)
			.unwrap(),
			MatchStr::EqualTo(string) => write_comparison(prefix, column, "=", string, sql),
			MatchStr::Not(query) => write_negated(prefix, column, query, sql),
			MatchStr::Or(queries) => write_boolean_group(prefix, column, queries, false, sql),
			MatchStr::Regex(regex) => write_comparison(prefix, column, "~", regex, sql),
		};
		true
	}
}
