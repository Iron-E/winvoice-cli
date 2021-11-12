use std::{
	fmt::{Display, Write},
	ops::Deref,
};

use clinvoice_adapter::WriteSqlWhereClause;
use clinvoice_query::{Match, MatchStr};

use super::PostgresSchema;

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
fn write_boolean_group<Q>(
	prefix: Option<&'static str>,
	column: &'static str,
	match_conditions: &[Q],
	union: bool,
	query: &mut String,
) where
	PostgresSchema: WriteSqlWhereClause<Q>,
{
	if let Some(p) = prefix
	{
		write!(query, " {}", p).unwrap()
	}

	if let Some(m) = match_conditions.first()
	{
		PostgresSchema::write_sql_where_clause(Some("("), column, m, query);
	}

	let separator = Some(if union { "AND" } else { "OR" });
	match_conditions.iter().skip(1).for_each(|q| {
		PostgresSchema::write_sql_where_clause(separator, column, q, query);
	});

	query.push(')');
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
	query: &mut String,
)
{
	write!(
		query,
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
	query: &mut String,
) where
	T: 't + Display,
{
	let mut iter = values.into_iter();

	if let Some(id) = iter.next()
	{
		write!(
			query,
			" {} {} = {}{}",
			if union { "ALL(ARRAY[" } else { "IN (" },
			prefix.unwrap_or_default(),
			column,
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
/// Wrap some `match_condition` in `NOT (…)`.
///
/// The args are the same as [`WriteSql::write_where`].
fn write_negated<Q>(
	prefix: Option<&'static str>,
	column: &'static str,
	match_condition: &Q,
	query: &mut String,
) where
	PostgresSchema: WriteSqlWhereClause<Q>,
{
	if let Some(p) = prefix
	{
		write!(query, " {}", p).unwrap()
	}

	PostgresSchema::write_sql_where_clause(Some("NOT ("), column, match_condition.deref(), query);

	query.push(')');
}

impl WriteSqlWhereClause<Match<'_, i64>> for PostgresSchema
{
	fn write_sql_where_clause(
		prefix: Option<&'static str>,
		column: &'static str,
		match_condition: &Match<'_, i64>,
		query: &mut String,
	) -> bool
	{
		match match_condition
		{
			Match::AllGreaterThan(id) | Match::GreaterThan(id) =>
			{
				write_comparison(prefix, column, ">", id, query)
			},
			Match::AllLessThan(id) | Match::LessThan(id) =>
			{
				write_comparison(prefix, column, "<", id, query)
			},
			Match::AllInRange(low, high) | Match::InRange(low, high) =>
			{
				write_comparison(prefix, column, ">=", low, query);
				write_comparison(Some("AND"), column, "<", high, query);
			},
			Match::And(match_conditions) =>
			{
				write_boolean_group(prefix, column, match_conditions, true, query)
			},
			Match::Any => return false,
			Match::EqualTo(id) => write_comparison(prefix, column, "=", id, query),
			Match::HasAll(ids) => write_has(prefix, column, ids, true, query),
			Match::HasAny(ids) => write_has(prefix, column, ids, false, query),
			Match::Not(match_condition) =>
			{
				write_negated(prefix, column, match_condition.deref(), query)
			},
			Match::Or(match_conditions) =>
			{
				write_boolean_group(prefix, column, match_conditions, false, query)
			},
		};
		true
	}
}

impl WriteSqlWhereClause<MatchStr<String>> for PostgresSchema
{
	fn write_sql_where_clause(
		prefix: Option<&'static str>,
		column: &'static str,
		match_condition: &MatchStr<String>,
		query: &mut String,
	) -> bool
	{
		match match_condition
		{
			MatchStr::And(match_conditions) =>
			{
				write_boolean_group(prefix, column, match_conditions, true, query)
			},
			MatchStr::Any => return false,
			MatchStr::Contains(string) => write!(
				query,
				" {} {} LIKE '%{}%'",
				prefix.unwrap_or_default(),
				column,
				string,
			)
			.unwrap(),
			MatchStr::EqualTo(string) => write_comparison(prefix, column, "=", string, query),
			MatchStr::Not(match_condition) =>
			{
				write_negated(prefix, column, match_condition.deref(), query)
			},
			MatchStr::Or(match_conditions) =>
			{
				write_boolean_group(prefix, column, match_conditions, false, query)
			},
			MatchStr::Regex(regex) => write_comparison(prefix, column, "~", regex, query),
		};
		true
	}
}
