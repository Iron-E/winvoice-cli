use std::{
	fmt::{Display, Write},
	ops::Deref,
};

use clinvoice_adapter::{WriteWhereClause, PREFIX_WHERE};
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
fn write_boolean_group<Q, const UNION: bool>(
	query: &mut String,
	prefix: &str,
	column: &str,
	match_conditions: &[Q],
) where
	PostgresSchema: WriteWhereClause<Q>,
{
	write!(query, " {} (", prefix).unwrap();
	if let Some(m) = match_conditions.first()
	{
		PostgresSchema::write_where_clause(true, column, m, query);
	}

	let separator: &str = if UNION { "AND" } else { "OR" };
	match_conditions.iter().skip(1).for_each(|q| {
		query.push_str(separator);
		PostgresSchema::write_where_clause(true, column, q, query);
	});

	query.push(')');
}

/// # Summary
///
/// Write a comparison of `column` and `comparand` using `comparator`.
///
/// The rest of the args are the same as [`WriteSql::write_where`].
fn write_comparison(
	query: &mut String,
	prefix: &str,
	column: &str,
	comperator: &str,
	comparand: impl Display,
)
{
	write!(query, " {} {} {} {}", prefix, column, comperator, comparand).unwrap()
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
	query: &mut String,
	prefix: &str,
	column: &str,
	values: impl IntoIterator<Item = &'t T>,
	union: bool,
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
			prefix,
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
	query: &mut String,
	prefix: &str,
	column: &str,
	match_condition: &Q,
) where
	PostgresSchema: WriteWhereClause<Q>,
{
	write!(query, " {} NOT (", prefix).unwrap();
	PostgresSchema::write_where_clause(true, column, match_condition, query);
	query.push(')');
}

impl WriteWhereClause<Match<'_, i64>> for PostgresSchema
{
	fn write_where_clause(
		keyword_written: bool,
		column: &str,
		match_condition: &Match<'_, i64>,
		query: &mut String,
	) -> bool
	{
		let prefix = if keyword_written { "" } else { PREFIX_WHERE };
		match match_condition
		{
			Match::AllGreaterThan(id) | Match::GreaterThan(id) =>
			{
				write_comparison(query, prefix, column, ">", id)
			},
			Match::AllLessThan(id) | Match::LessThan(id) =>
			{
				write_comparison(query, prefix, column, "<", id)
			},
			Match::AllInRange(low, high) | Match::InRange(low, high) =>
			{
				write_comparison(query, prefix, column, ">=", low);
				write_comparison(query, "AND", column, "<", high);
			},
			Match::And(match_conditions) =>
			{
				write_boolean_group::<_, true>(query, prefix, column, match_conditions)
			},
			Match::Any => return false,
			Match::EqualTo(id) => write_comparison(query, prefix, column, "=", id),
			Match::HasAll(ids) => write_has(query, prefix, column, ids, true),
			Match::HasAny(ids) => write_has(query, prefix, column, ids, false),
			Match::Not(match_condition) => match match_condition.deref()
			{
				Match::Any => write!(query, " {} {} IS NULL", prefix, column).unwrap(),
				m @ _ => write_negated(query, prefix, column, m),
			},
			Match::Or(match_conditions) =>
			{
				write_boolean_group::<_, false>(query, prefix, column, match_conditions)
			},
		};
		true
	}
}

impl WriteWhereClause<MatchStr<String>> for PostgresSchema
{
	fn write_where_clause(
		keyword_written: bool,
		column: &str,
		match_condition: &MatchStr<String>,
		query: &mut String,
	) -> bool
	{
		let prefix = if keyword_written { "" } else { PREFIX_WHERE };
		match match_condition
		{
			MatchStr::And(match_conditions) =>
			{
				write_boolean_group::<_, true>(prefix, column, match_conditions, query)
			},
			MatchStr::Any => return false,
			MatchStr::Contains(string) =>
			{
				write!(query, " {} {} LIKE '%{}%'", prefix, column, string,).unwrap()
			},
			MatchStr::EqualTo(string) => write_comparison(prefix, column, "=", string, query),
			MatchStr::Not(match_condition) =>
			{
				write_negated(prefix, column, match_condition.deref(), query)
			},
			MatchStr::Or(match_conditions) =>
			{
				write_boolean_group::<_, false>(prefix, column, match_conditions, query)
			},
			MatchStr::Regex(regex) => write_comparison(prefix, column, "~", regex, query),
		};
		true
	}
}
