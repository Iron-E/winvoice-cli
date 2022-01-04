use core::{
	fmt::{Display, Write},
	ops::Deref,
};
use std::borrow::Cow;

use clinvoice_adapter::{WriteContext, WriteWhereClause};
use clinvoice_finance::Money;
use clinvoice_match::{Match, MatchEmployee, MatchJob, MatchOrganization, MatchPerson, MatchStr};
use clinvoice_schema::chrono::NaiveDateTime;

use super::{PgSchema as Schema, PostgresDateTime, PostgresOption, PostgresStr, PostgresTypeCast};

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
fn write_boolean_group<D, I, Q, const UNION: bool>(
	query: &mut String,
	context: WriteContext,
	alias: D,
	match_conditions: &mut I,
) where
	D: Copy + Display,
	I: Iterator<Item = Q>,
	Schema: WriteWhereClause<Q>,
{
	write!(query, "{} (", context.get_prefix()).unwrap();
	if let Some(m) = match_conditions.next()
	{
		Schema::write_where_clause(WriteContext::InWhereCondition, alias, m, query);
	}

	let separator = if UNION { " AND" } else { " OR" };
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
	alias: impl Copy + Display,
	comperator: &str,
	comparand: impl Copy + Display,
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
fn write_has<T>(
	query: &mut String,
	context: WriteContext,
	alias: impl Copy + Display,
	values: impl IntoIterator<Item = T>,
	union: bool,
) where
	T: Copy + Display,
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
fn write_is_null(query: &mut String, context: WriteContext, alias: impl Copy + Display)
{
	write!(query, "{} {} IS NULL", context.get_prefix(), alias).unwrap()
}

/// # Summary
///
/// Wrap some `match_condition` in `NOT (…)`.
///
/// The args are the same as [`WriteSql::write_where`].
fn write_negated<Q>(
	query: &mut String,
	context: WriteContext,
	alias: impl Copy + Display,
	match_condition: Q,
) where
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
		alias: impl Copy + Display,
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
				write_comparison(query, context, alias, "BETWEEN", low);
				write_comparison(query, WriteContext::InWhereCondition, "", "AND", high);
			},
			Match::And(match_conditions) => write_boolean_group::<_, _, _, true>(
				query,
				context,
				alias,
				&mut match_conditions.iter().filter(|m| *m != &Match::Any),
			),
			Match::Any => return context,
			Match::EqualTo(id) => write_comparison(query, context, alias, "=", id),
			Match::HasAll(ids) => write_has(query, context, alias, ids.deref(), true),
			Match::HasAny(ids) => write_has(query, context, alias, ids.deref(), false),
			Match::Not(match_condition) => match match_condition.deref()
			{
				Match::Any => write_is_null(query, context, alias),
				m => write_negated(query, context, alias, m),
			},
			Match::Or(match_conditions) => write_boolean_group::<_, _, _, false>(
				query,
				context,
				alias,
				&mut match_conditions.iter().filter(|m| *m != &Match::Any),
			),
		};
		WriteContext::AfterWhereCondition
	}
}

impl WriteWhereClause<&Match<'_, Money>> for Schema
{
	fn write_where_clause(
		context: WriteContext,
		alias: impl Copy + Display,
		match_condition: &Match<'_, Money>,
		query: &mut String,
	) -> WriteContext
	{
		let alias_cast = PostgresTypeCast::numeric(alias);
		match match_condition
		{
			Match::AllGreaterThan(money) | Match::GreaterThan(money) =>
			{
				write_comparison(query, context, alias_cast, ">", money.amount)
			},
			Match::AllLessThan(money) | Match::LessThan(money) =>
			{
				write_comparison(query, context, alias_cast, "<", money)
			},
			Match::AllInRange(low, high) | Match::InRange(low, high) =>
			{
				write_comparison(query, context, alias_cast, "BETWEEN", low);
				write_comparison(query, WriteContext::InWhereCondition, "", "AND", high);
			},
			Match::And(match_conditions) => write_boolean_group::<_, _, _, true>(
				query,
				context,
				alias_cast,
				&mut match_conditions.iter().filter(|m| *m != &Match::Any),
			),
			Match::Any => return context,
			Match::EqualTo(money) => write_comparison(query, context, alias_cast, "=", money),
			Match::HasAll(moneys) => write_has(query, context, alias_cast, moneys.deref(), true),
			Match::HasAny(moneys) => write_has(query, context, alias_cast, moneys.deref(), false),
			Match::Not(match_condition) => match match_condition.deref()
			{
				Match::Any => write_is_null(query, context, alias_cast),
				m => write_negated(query, context, alias_cast, m),
			},
			Match::Or(match_conditions) => write_boolean_group::<_, _, _, false>(
				query,
				context,
				alias_cast,
				&mut match_conditions.iter().filter(|m| *m != &Match::Any),
			),
		};
		WriteContext::AfterWhereCondition
	}
}

impl WriteWhereClause<&Match<'_, NaiveDateTime>> for Schema
{
	fn write_where_clause(
		context: WriteContext,
		alias: impl Copy + Display,
		match_condition: &Match<'_, NaiveDateTime>,
		query: &mut String,
	) -> WriteContext
	{
		match match_condition
		{
			Match::AllGreaterThan(date) | Match::GreaterThan(date) =>
			{
				write_comparison(query, context, alias, ">", PostgresDateTime(**date))
			},
			Match::AllLessThan(date) | Match::LessThan(date) =>
			{
				write_comparison(query, context, alias, "<", PostgresDateTime(**date))
			},
			Match::AllInRange(low, high) | Match::InRange(low, high) =>
			{
				write_comparison(query, context, alias, "BETWEEN", PostgresDateTime(**low));
				write_comparison(
					query,
					WriteContext::InWhereCondition,
					"",
					"AND",
					PostgresDateTime(**high),
				);
			},
			Match::And(match_conditions) => write_boolean_group::<_, _, _, true>(
				query,
				context,
				alias,
				&mut match_conditions.iter().filter(|m| *m != &Match::Any),
			),
			Match::Any => return context,
			Match::EqualTo(date) =>
			{
				write_comparison(query, context, alias, "=", PostgresDateTime(**date))
			},
			Match::HasAll(dates) => write_has(
				query,
				context,
				alias,
				dates.iter().copied().map(PostgresDateTime),
				true,
			),
			Match::HasAny(dates) => write_has(
				query,
				context,
				alias,
				dates.iter().copied().map(PostgresDateTime),
				false,
			),
			Match::Not(match_condition) => match match_condition.deref()
			{
				Match::Any => write_is_null(query, context, alias),
				m => write_negated(query, context, alias, m),
			},
			Match::Or(match_conditions) => write_boolean_group::<_, _, _, false>(
				query,
				context,
				alias,
				&mut match_conditions.iter().filter(|m| *m != &Match::Any),
			),
		};
		WriteContext::AfterWhereCondition
	}
}

impl WriteWhereClause<&Match<'_, Option<NaiveDateTime>>> for Schema
{
	fn write_where_clause(
		context: WriteContext,
		alias: impl Copy + Display,
		match_condition: &Match<'_, Option<NaiveDateTime>>,
		query: &mut String,
	) -> WriteContext
	{
		match match_condition
		{
			Match::AllGreaterThan(date) | Match::GreaterThan(date) => write_comparison(
				query,
				context,
				alias,
				">",
				PostgresOption(date.map(PostgresDateTime)),
			),
			Match::AllLessThan(date) | Match::LessThan(date) => write_comparison(
				query,
				context,
				alias,
				"<",
				PostgresOption(date.map(PostgresDateTime)),
			),
			Match::AllInRange(low, high) | Match::InRange(low, high) =>
			{
				write_comparison(
					query,
					context,
					alias,
					"BETWEEN",
					PostgresOption(low.map(PostgresDateTime)),
				);
				write_comparison(
					query,
					WriteContext::InWhereCondition,
					"",
					"AND",
					PostgresOption(high.map(PostgresDateTime)),
				);
			},
			Match::And(match_conditions) => write_boolean_group::<_, _, _, true>(
				query,
				context,
				alias,
				&mut match_conditions.iter().filter(|m| *m != &Match::Any),
			),
			Match::Any => return context,
			Match::EqualTo(date) => write_comparison(
				query,
				context,
				alias,
				"=",
				PostgresOption(date.map(PostgresDateTime)),
			),
			Match::HasAll(dates) => write_has(
				query,
				context,
				alias,
				dates
					.iter()
					.map(|o| PostgresOption(o.map(PostgresDateTime))),
				true,
			),
			Match::HasAny(dates) => write_has(
				query,
				context,
				alias,
				dates
					.iter()
					.map(|o| PostgresOption(o.map(PostgresDateTime))),
				false,
			),
			Match::Not(match_condition) => match match_condition.deref()
			{
				Match::Any => write_is_null(query, context, alias),
				m => write_negated(query, context, alias, m),
			},
			Match::Or(match_conditions) => write_boolean_group::<_, _, _, false>(
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
	///        Might be able to fix by replacing `'` with `''` before entering.
	fn write_where_clause(
		context: WriteContext,
		alias: impl Copy + Display,
		match_condition: &MatchStr<Cow<'_, str>>,
		query: &mut String,
	) -> WriteContext
	{
		match match_condition
		{
			MatchStr::And(match_conditions) => write_boolean_group::<_, _, _, true>(
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
				write_comparison(query, context, alias, "=", PostgresStr(string))
			},
			MatchStr::Not(match_condition) => match match_condition.deref()
			{
				MatchStr::Any => write_is_null(query, context, alias),
				m => write_negated(query, context, alias, m),
			},
			MatchStr::Or(match_conditions) => write_boolean_group::<_, _, _, false>(
				query,
				context,
				alias,
				&mut match_conditions.iter().filter(|m| *m != &MatchStr::Any),
			),
			MatchStr::Regex(regex) => write_comparison(query, context, alias, "~", PostgresStr(regex)),
		};
		WriteContext::AfterWhereCondition
	}
}

impl WriteWhereClause<&MatchPerson<'_>> for Schema
{
	fn write_where_clause(
		context: WriteContext,
		alias: impl Copy + Display,
		match_condition: &MatchPerson,
		query: &mut String,
	) -> WriteContext
	{
		macro_rules! write_where_clause {
			($context:expr, $column:expr, $match_field:ident) => {
				Schema::write_where_clause(
					$context,
					&format!("{}.{}", alias, $column),
					&match_condition.$match_field,
					query,
				)
			};
		}

		write_where_clause!(write_where_clause!(context, "id", id), "name", name)
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
		alias: impl Copy + Display,
		match_condition: &MatchOrganization,
		query: &mut String,
	) -> WriteContext
	{
		Schema::write_where_clause(
			Schema::write_where_clause(
				context,
				&format!("{}.id", alias),
				&match_condition.id,
				query,
			),
			&format!("{}.name", alias),
			&match_condition.name,
			query,
		)
	}
}

impl WriteWhereClause<&MatchEmployee<'_>> for Schema
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
		alias: impl Copy + Display,
		match_condition: &MatchEmployee,
		query: &mut String,
	) -> WriteContext
	{
		Schema::write_where_clause(
			Schema::write_where_clause(
				Schema::write_where_clause(
					Schema::write_where_clause(
						Schema::write_where_clause(context, "O", &match_condition.organization, query),
						"P",
						&match_condition.person,
						query,
					),
					&format!("{}.id", alias),
					&match_condition.id,
					query,
				),
				&format!("{}.status", alias),
				&match_condition.status,
				query,
			),
			&format!("{}.title", alias),
			&match_condition.title,
			query,
		)
	}
}

impl WriteWhereClause<&MatchJob<'_>> for Schema
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
		alias: impl Copy + Display,
		match_condition: &MatchJob,
		query: &mut String,
	) -> WriteContext
	{
		Schema::write_where_clause(
			Schema::write_where_clause(
				Schema::write_where_clause(
					Schema::write_where_clause(
						Schema::write_where_clause(
							// Schema::write_where_clause(
							Schema::write_where_clause(
								Schema::write_where_clause(
									Schema::write_where_clause(
										context,
										&format!("{}.id", alias),
										&match_condition.id,
										query,
									),
									&format!("{}.date_close", alias),
									&match_condition.date_close,
									query,
								),
								&format!("{}.date_open", alias),
								&match_condition.date_open,
								query,
							),
							// &format!("{}.increment", alias),
							// &match_condition.increment,
							// query,
							// ),
							&format!("{}.invoice_date_issued", alias),
							&match_condition.invoice.date_issued,
							query,
						),
						&format!("{}.invoice_date_paid", alias),
						&match_condition.invoice.date_paid,
						query,
					),
					&format!("{}.invoice_hourly_rate", alias),
					&match_condition.invoice.hourly_rate,
					query,
				),
				&format!("{}.notes", alias),
				&match_condition.notes,
				query,
			),
			&format!("{}.objectives", alias),
			&match_condition.objectives,
			query,
		)
	}
}

#[cfg(test)]
mod tests
{
	use std::borrow::Cow::{Borrowed, Owned};

	use clinvoice_match::{MatchLocation, MatchOuterLocation};

	use super::{
		Match,
		MatchOrganization,
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
			Schema::write_where_clause(BeforeWhereClause, "foo", &Match::from(18), &mut query),
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
					Match::HasAny(Borrowed(&[0, 9, 7, 4])),
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
				" ( NOT ( bar BETWEEN 0  AND 10) AND bar IN ({}) AND ( bar IS NULL OR bar > -1))",
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
					&Match::<i64>::AllInRange(Owned(0), Owned(2)),
					&mut query,
				),
				AfterWhereCondition,
			);
			assert_eq!(
				Schema::write_where_clause(
					InWhereCondition,
					"another_row",
					&Match::<i64>::InRange(Owned(0), Owned(2)),
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
					&Match::<i64>::AllLessThan(Owned(0)),
					&mut query,
				),
				AfterWhereCondition,
			);
			assert_eq!(
				Schema::write_where_clause(
					InWhereCondition,
					"another_row",
					&Match::<i64>::LessThan(Owned(0)),
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
					&Match::<i64>::AllGreaterThan(Owned(0)),
					&mut query,
				),
				AfterWhereCondition,
			);
			assert_eq!(
				Schema::write_where_clause(
					InWhereCondition,
					"another_row",
					&Match::<i64>::GreaterThan(Owned(0)),
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
					MatchStr::Not(Box::new("not equal".into())),
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
					id: 7.into(),
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
					id: 7.into(),
					name: "stuff".into(),
				},
				&mut query,
			),
			AfterWhereCondition
		);
		assert_eq!(query, String::from(" AND id = 7 AND name = 'stuff'"));
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
					id: 7.into(),
					name: MatchStr::Contains(Borrowed("Gögle")),
					location: MatchLocation {
						id: 11.into(),
						outer: MatchOuterLocation::Some(Box::new(MatchLocation {
							id: 14.into(),
							outer: MatchOuterLocation::Some(Box::new(MatchLocation {
								name: "Japan".into(),
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
			String::from(" WHERE O.id = 7 AND O.name LIKE '%Gögle%'")
		);
	}
}
