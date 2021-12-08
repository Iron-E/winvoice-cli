use std::{
	borrow::Cow,
	fmt::{Display, Write},
	ops::Deref,
};

use clinvoice_adapter::{WriteContext, WriteJoinClause, WriteWhereClause};
use clinvoice_match::{
	Match,
	MatchLocation,
	MatchOrganization,
	MatchOuterLocation,
	MatchPerson,
	MatchStr,
};

use super::PostgresSchema as Schema;

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
fn write_boolean_group<I, Q, const UNION: bool>(
	query: &mut String,
	context: WriteContext,
	alias: &str,
	match_conditions: &mut I,
) where
	I: Iterator<Item = Q>,
	Schema: WriteWhereClause<Q>,
{
	write!(query, "{} (", context.get_prefix()).unwrap();
	if let Some(m) = match_conditions.next()
	{
		Schema::write_where_clause(WriteContext::InWhereCondition, alias, m, query);
	}

	let separator: &str = if UNION { " AND" } else { " OR" };
	match_conditions.for_each(|q| {
		query.push_str(separator);
		Schema::write_where_clause(WriteContext::InWhereCondition, alias, q, query);
	});

	query.push(')');
}

/// # Summary
///
/// Write a comparison of `alias` and `comparand` using `comparator`.
///
/// The rest of the args are the same as [`WriteSql::write_where`].
fn write_comparison(
	query: &mut String,
	context: WriteContext,
	alias: &str,
	comperator: &str,
	comparand: impl Display,
)
{
	write!(
		query,
		"{} {} {} {}",
		context.get_prefix(),
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
fn write_has<'t, T>(
	query: &mut String,
	context: WriteContext,
	alias: &str,
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
			"{} {} {}{}",
			context.get_prefix(),
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
fn write_is_null(query: &mut String, context: WriteContext, alias: &str)
{
	write!(query, "{} {} IS NULL", context.get_prefix(), alias).unwrap()
}

/// # Summary
///
/// Wrap some `match_condition` in `NOT (…)`.
///
/// The args are the same as [`WriteSql::write_where`].
fn write_negated<Q>(query: &mut String, context: WriteContext, alias: &str, match_condition: Q)
where
	Schema: WriteWhereClause<Q>,
{
	write!(query, "{} NOT (", context.get_prefix()).unwrap();
	Schema::write_where_clause(
		WriteContext::InWhereCondition,
		alias,
		match_condition,
		query,
	);
	query.push(')');
}

impl WriteWhereClause<&Match<'_, i64>> for Schema
{
	fn write_where_clause(
		context: WriteContext,
		alias: &str,
		match_condition: &Match<'_, i64>,
		query: &mut String,
	) -> WriteContext
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
				write_comparison(query, WriteContext::AfterWhereCondition, alias, "<", high);
			},
			Match::And(match_conditions) => write_boolean_group::<_, _, true>(
				query,
				context,
				alias,
				&mut match_conditions.iter().filter(|m| *m != &Match::Any),
			),
			Match::Any => return context,
			Match::EqualTo(id) => write_comparison(query, context, alias, "=", id),
			Match::HasAll(ids) => write_has(query, context, alias, ids, true),
			Match::HasAny(ids) => write_has(query, context, alias, ids, false),
			Match::Not(match_condition) => match match_condition.deref()
			{
				Match::Any => write_is_null(query, context, alias),
				m @ _ => write_negated(query, context, alias, m),
			},
			Match::Or(match_conditions) => write_boolean_group::<_, _, false>(
				query,
				context,
				alias,
				&mut match_conditions.iter().filter(|m| *m != &Match::Any),
			),
		};
		WriteContext::AfterWhereCondition
	}
}

impl WriteWhereClause<&MatchStr<Cow<'_, str>>> for Schema
{
	/// FIXME: `MatchStr::EqualTo("Foo's Place")` would break this, because of the apostraphe.
	fn write_where_clause(
		context: WriteContext,
		alias: &str,
		match_condition: &MatchStr<Cow<'_, str>>,
		query: &mut String,
	) -> WriteContext
	{
		match match_condition
		{
			MatchStr::And(match_conditions) => write_boolean_group::<_, _, true>(
				query,
				context,
				alias,
				&mut match_conditions.iter().filter(|m| *m != &MatchStr::Any),
			),
			MatchStr::Any => return context,
			MatchStr::Contains(string) => write!(
				query,
				"{} {} LIKE '%{}%'",
				context.get_prefix(),
				alias,
				string
			)
			.unwrap(),
			MatchStr::EqualTo(string) =>
			{
				write!(query, "{} {} = '{}'", context.get_prefix(), alias, string).unwrap()
			},
			MatchStr::Not(match_condition) => match match_condition.deref()
			{
				MatchStr::Any => write_is_null(query, context, alias),
				m @ _ => write_negated(query, context, alias, m),
			},
			MatchStr::Or(match_conditions) => write_boolean_group::<_, _, false>(
				query,
				context,
				alias,
				&mut match_conditions.iter().filter(|m| *m != &MatchStr::Any),
			),
			MatchStr::Regex(regex) => write_comparison(query, context, alias, "~", regex),
		};
		WriteContext::AfterWhereCondition
	}
}

impl WriteWhereClause<&MatchPerson<'_>> for Schema
{
	fn write_where_clause(
		context: WriteContext,
		alias: &str,
		match_condition: &MatchPerson,
		query: &mut String,
	) -> WriteContext
	{
		macro_rules! write_where_clause {
			($context:expr, $column:ident, $match_field:ident) => {
				if alias.is_empty()
				{
					Schema::write_where_clause($context, $column, &match_condition.$match_field, query)
				}
				else
				{
					Schema::write_where_clause(
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

impl WriteWhereClause<&MatchLocation<'_>> for Schema
{
	/// # Panics
	///
	/// If any the following:
	///
	/// * `context` is not `BeforeWhereClause`
	/// * `alias` is an empty string.
	///
	/// # See
	///
	/// * [`WriteWhereClause::write_where_clause`]
	fn write_where_clause(
		context: WriteContext,
		alias: &str,
		match_condition: &MatchLocation,
		query: &mut String,
	) -> WriteContext
	{
		fn recurse(
			query: &mut String,
			context: WriteContext,
			alias: &str,
			match_condition: &MatchLocation,
		) -> WriteContext
		{
			let ctx = match match_condition.outer
			{
				MatchOuterLocation::Any => context,
				MatchOuterLocation::None =>
				{
					write_is_null(query, context, &format!("{}.{}", alias, COLUMN_OUTER_ID));
					WriteContext::AfterWhereCondition
				},
				MatchOuterLocation::Some(ref outer) if context == WriteContext::BeforeWhereClause =>
				{
					let new_alias = format!("{}O", alias);
					Schema::write_join_clause(
						query,
						"",
						"locations",
						&new_alias,
						"id",
						&format!("{}.{}", alias, COLUMN_OUTER_ID),
					)
					.unwrap();
					recurse(query, context, &new_alias, outer.deref())
				},
				MatchOuterLocation::Some(_) => panic!(
					"Must generate SQL for `MatchLocation` _before_ the `WHERE` condition, as it \
					 necessitates a JOIN."
				),
			};

			Schema::write_where_clause(
				Schema::write_where_clause(
					ctx,
					&format!("{}.{}", alias, COLUMN_ID),
					&match_condition.id,
					query,
				),
				&format!("{}.{}", alias, COLUMN_NAME),
				&match_condition.name,
				query,
			)
		}

		if alias.is_empty()
		{
			panic!("Must provide alias for `Location`");
		}

		recurse(query, context, alias, match_condition)
	}
}

impl WriteWhereClause<&MatchOrganization<'_>> for Schema
{
	/// # Panics
	///
	/// If any the following:
	///
	/// * `context` is not `BeforeWhereClause`
	/// * `alias` is an empty string.
	///
	/// # See
	///
	/// * [`WriteWhereClause::write_where_clause`]
	fn write_where_clause(
		context: WriteContext,
		alias: &str,
		match_condition: &MatchOrganization,
		query: &mut String,
	) -> WriteContext
	{
		Schema::write_where_clause(
			Schema::write_where_clause(
				Schema::write_where_clause(context, "L", &match_condition.location, query),
				&format!("{}.{}", alias, COLUMN_ID),
				&match_condition.id,
				query,
			),
			&format!("{}.{}", alias, COLUMN_NAME),
			&match_condition.name,
			query,
		)
	}
}

#[cfg(test)]
mod tests
{
	use std::borrow::Cow::{Borrowed, Owned};

	use clinvoice_match::MatchOrganization;

	use super::{
		Match,
		MatchLocation,
		MatchOuterLocation,
		MatchPerson,
		MatchStr,
		Schema,
		WriteContext::{AfterWhereCondition, BeforeWhereClause, InWhereCondition},
		WriteWhereClause,
	};

	#[test]
	fn write_match_where_clause()
	{
		let mut query = String::new();
		assert_eq!(
			Schema::write_where_clause(
				BeforeWhereClause,
				"foo",
				&Match::EqualTo(Owned(18)),
				&mut query
			),
			AfterWhereCondition,
		);
		assert_eq!(query, String::from(" WHERE foo = 18"));

		query.clear();
		assert_eq!(
			Schema::write_where_clause(
				InWhereCondition,
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
			),
			AfterWhereCondition,
		);
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

			assert_eq!(
				Schema::write_where_clause(
					InWhereCondition,
					"another_row",
					&Match::AllInRange(Owned(0), Owned(2)),
					&mut query,
				),
				AfterWhereCondition,
			);
			assert_eq!(
				Schema::write_where_clause(
					InWhereCondition,
					"another_row",
					&Match::InRange(Owned(0), Owned(2)),
					&mut query2,
				),
				AfterWhereCondition,
			);
			assert_eq!(query, query2);

			query.clear();
			query2.clear();
			assert_eq!(
				Schema::write_where_clause(
					InWhereCondition,
					"another_row",
					&Match::AllLessThan(Owned(0)),
					&mut query,
				),
				AfterWhereCondition,
			);
			assert_eq!(
				Schema::write_where_clause(
					InWhereCondition,
					"another_row",
					&Match::LessThan(Owned(0)),
					&mut query2,
				),
				AfterWhereCondition,
			);
			assert_eq!(query, query2);

			query.clear();
			query2.clear();
			assert_eq!(
				Schema::write_where_clause(
					InWhereCondition,
					"another_row",
					&Match::AllGreaterThan(Owned(0)),
					&mut query,
				),
				AfterWhereCondition,
			);
			assert_eq!(
				Schema::write_where_clause(
					InWhereCondition,
					"another_row",
					&Match::GreaterThan(Owned(0)),
					&mut query2,
				),
				AfterWhereCondition,
			);
			assert_eq!(query, query2);
		}
	}

	#[test]
	fn write_match_str_where_clause()
	{
		let mut query = String::new();
		assert_eq!(
			Schema::write_where_clause(InWhereCondition, "bar", &MatchStr::Any, &mut query),
			InWhereCondition
		);
		assert_eq!(query, String::from(""));

		query.clear();
		assert_eq!(
			Schema::write_where_clause(
				AfterWhereCondition,
				"bar",
				&MatchStr::Contains(Borrowed("punky brüster")),
				&mut query,
			),
			AfterWhereCondition,
		);
		assert_eq!(query, String::from(" AND bar LIKE '%punky brüster%'"));

		query.clear();
		assert_eq!(
			Schema::write_where_clause(
				BeforeWhereClause,
				"some_row",
				&MatchStr::Or(vec![
					MatchStr::Regex(Borrowed(r#"^f.rk.*\bit\b.*over$"#)),
					MatchStr::Not(Box::new(MatchStr::EqualTo(Borrowed("not equal")))),
				]),
				&mut query,
			),
			AfterWhereCondition,
		);
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
		assert_eq!(
			Schema::write_where_clause(BeforeWhereClause, "", &MatchPerson::default(), &mut query),
			BeforeWhereClause
		);
		assert!(query.is_empty());

		query.clear();
		assert_eq!(
			Schema::write_where_clause(
				BeforeWhereClause,
				"",
				&MatchPerson {
					id: Match::EqualTo(Owned(7)),
					..Default::default()
				},
				&mut query,
			),
			AfterWhereCondition
		);
		assert_eq!(query, String::from(" WHERE id = 7"));

		query.clear();
		assert_eq!(
			Schema::write_where_clause(
				AfterWhereCondition,
				"",
				&MatchPerson {
					id: Match::EqualTo(Owned(7)),
					name: MatchStr::EqualTo(Borrowed("stuff")),
				},
				&mut query,
			),
			AfterWhereCondition
		);
		assert_eq!(query, String::from(" AND id = 7 AND name = 'stuff'"));
	}

	#[test]
	fn write_location_join_where_clause()
	{
		let mut query = String::new();
		assert_eq!(
			Schema::write_where_clause(
				BeforeWhereClause,
				"L",
				&MatchLocation::default(),
				&mut query
			),
			BeforeWhereClause
		);
		assert!(query.is_empty());

		query.clear();
		assert_eq!(
			Schema::write_where_clause(
				BeforeWhereClause,
				"L",
				&MatchLocation {
					id: Match::EqualTo(Owned(7)),
					..Default::default()
				},
				&mut query,
			),
			AfterWhereCondition
		);
		assert_eq!(query, String::from(" WHERE L.id = 7"));

		query.clear();
		assert_eq!(
			Schema::write_where_clause(
				BeforeWhereClause,
				"L",
				&MatchLocation {
					id: Match::EqualTo(Owned(7)),
					outer: MatchOuterLocation::Some(Box::new(MatchLocation {
						id: Match::EqualTo(Owned(8)),
						outer: MatchOuterLocation::Some(Box::new(MatchLocation {
							id: Match::EqualTo(Owned(9)),
							outer: MatchOuterLocation::None,
							..Default::default()
						})),
						..Default::default()
					})),
					..Default::default()
				},
				&mut query,
			),
			AfterWhereCondition
		);
		assert_eq!(
			query,
			String::from(
				"  JOIN locations LO ON (LO.id = L.outer_id)  JOIN locations LOO ON (LOO.id = \
				 LO.outer_id) WHERE LOO.outer_id IS NULL AND LOO.id = 9 AND LO.id = 8 AND L.id = 7"
			),
		);
	}

	#[test]
	fn write_organization_where_clause()
	{
		let mut query = String::new();
		assert_eq!(
			Schema::write_where_clause(
				BeforeWhereClause,
				"O",
				&MatchOrganization::default(),
				&mut query
			),
			BeforeWhereClause
		);
		assert!(query.is_empty());

		query.clear();
		assert_eq!(
			Schema::write_where_clause(
				BeforeWhereClause,
				"O",
				&MatchOrganization {
					id: Match::EqualTo(Owned(7)),
					name: MatchStr::Contains(Borrowed("Gögle")),
					location: MatchLocation {
						id: Match::EqualTo(Owned(11)),
						outer: MatchOuterLocation::Some(Box::new(MatchLocation {
							id: Match::EqualTo(Owned(14)),
							outer: MatchOuterLocation::Some(Box::new(MatchLocation {
								name: MatchStr::EqualTo(Borrowed("Japan")),
								..Default::default()
							})),
							..Default::default()
						})),
						..Default::default()
					},
				},
				&mut query,
			),
			AfterWhereCondition
		);
		assert_eq!(
			query,
			String::from(
				"  JOIN locations LO ON (LO.id = L.outer_id)  JOIN locations LOO ON (LOO.id = \
				 LO.outer_id) WHERE LOO.name = 'Japan' AND LO.id = 14 AND L.id = 11 AND O.id = 7 AND \
				 O.name LIKE '%Gögle%'"
			)
		);
	}
}
