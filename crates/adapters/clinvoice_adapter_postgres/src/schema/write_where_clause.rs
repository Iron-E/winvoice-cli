use core::{
	fmt::{Display, Write},
	ops::Deref,
	time::Duration,
};

use clinvoice_adapter::{WriteContext, WriteWhereClause};
use clinvoice_finance::Money;
use clinvoice_match::{
	Match,
	MatchEmployee,
	MatchExpense,
	MatchJob,
	MatchOrganization,
	MatchPerson,
	MatchStr,
	MatchTimesheet,
	Serde,
};
use clinvoice_schema::chrono::NaiveDateTime;

use super::{PgInterval, PgOption, PgSchema as Schema, PgStr, PgTimestampTz};

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
///       [`std::borrow::Cow`].  the linked documentation for proper examples.
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
	write!(query, "{context} (").unwrap();
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
	comparator: &str,
	comparand: impl Copy + Display,
)
{
	write!(query, "{context} {alias} {comparator} {comparand}").unwrap()
}

/// # Summary
///
/// Check if some `alias` has `ANY` or `ALL` of the `values` provided.
///
/// * If `union` is `true`, a check is done to see if `ALL` of the `values` are `alias`.
/// * If `union` is `false`, a check is done to see if `ANY` of the `values` are `alias`.
///
/// NOTE: the above is not proper [`Match`] syntax, since they need to wrap their inner value in a
///       [`std::borrow::Cow`].  the linked documentation for proper examples.
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
			"{context} {alias} {}{id}",
			if union { "= ALL(ARRAY[" } else { "IN (" },
		)
		.unwrap()
	}

	iter.for_each(|id| write!(query, ", {id}").unwrap());

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
	write!(query, "{context} {alias} IS NULL").unwrap()
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
	write!(query, "{context} NOT (").unwrap();
	Schema::write_where_clause(
		WriteContext::InWhereCondition,
		alias,
		match_condition,
		query,
	);
	query.push(')');
}

impl WriteWhereClause<&Match<Serde<Duration>>> for Schema
{
	fn write_where_clause(
		context: WriteContext,
		alias: impl Copy + Display,
		match_condition: &Match<Serde<Duration>>,
		query: &mut String,
	) -> WriteContext
	{
		match match_condition
		{
			Match::AllGreaterThan(duration) | Match::GreaterThan(duration) => write_comparison(
				query,
				context,
				alias,
				">",
				PgInterval(duration.into_inner()),
			),
			Match::AllLessThan(duration) | Match::LessThan(duration) => write_comparison(
				query,
				context,
				alias,
				"<",
				PgInterval(duration.into_inner()),
			),
			Match::AllInRange(low, high) | Match::InRange(low, high) =>
			{
				write_comparison(
					query,
					context,
					alias,
					"BETWEEN",
					PgInterval(low.into_inner()),
				);
				write_comparison(
					query,
					WriteContext::InWhereCondition,
					"",
					"AND",
					PgInterval(high.into_inner()),
				);
			},
			Match::And(match_conditions) => write_boolean_group::<_, _, _, true>(
				query,
				context,
				alias,
				&mut match_conditions.iter().filter(|m| *m != &Match::Any),
			),
			Match::Any => return context,
			Match::EqualTo(duration) => write_comparison(
				query,
				context,
				alias,
				"=",
				PgInterval(duration.into_inner()),
			),
			Match::HasAll(durations) => write_has(
				query,
				context,
				alias,
				durations.iter().map(|d| PgInterval(d.into_inner())),
				true,
			),
			Match::HasAny(durations) => write_has(
				query,
				context,
				alias,
				durations.iter().map(|d| PgInterval(d.into_inner())),
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

impl WriteWhereClause<&Match<i64>> for Schema
{
	fn write_where_clause(
		context: WriteContext,
		alias: impl Copy + Display,
		match_condition: &Match<i64>,
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
			Match::HasAll(ids) => write_has(query, context, alias, ids, true),
			Match::HasAny(ids) => write_has(query, context, alias, ids, false),
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

impl WriteWhereClause<&Match<Money>> for Schema
{
	fn write_where_clause(
		context: WriteContext,
		alias: impl Copy + Display,
		match_condition: &Match<Money>,
		query: &mut String,
	) -> WriteContext
	{
		// TODO: use `PgTypecast::numeric(alias)` after rust-lang/rust#39959
		let alias_cast = format!("{alias}::numeric"); // PgTypeCast::numeric(alias);
		match match_condition
		{
			Match::AllGreaterThan(money) | Match::GreaterThan(money) =>
			{
				write_comparison(query, context, &alias_cast, ">", money.amount)
			},
			Match::AllLessThan(money) | Match::LessThan(money) =>
			{
				write_comparison(query, context, &alias_cast, "<", money)
			},
			Match::AllInRange(low, high) | Match::InRange(low, high) =>
			{
				write_comparison(query, context, &alias_cast, "BETWEEN", low);
				write_comparison(query, WriteContext::InWhereCondition, "", "AND", high);
			},
			Match::And(match_conditions) => write_boolean_group::<_, _, _, true>(
				query,
				context,
				&alias_cast,
				&mut match_conditions.iter().filter(|m| *m != &Match::Any),
			),
			Match::Any => return context,
			Match::EqualTo(money) => write_comparison(query, context, &alias_cast, "=", money),
			Match::HasAll(moneys) => write_has(query, context, &alias_cast, moneys, true),
			Match::HasAny(moneys) => write_has(query, context, &alias_cast, moneys, false),
			Match::Not(match_condition) => match match_condition.deref()
			{
				Match::Any => write_is_null(query, context, &alias_cast),
				m => write_negated(query, context, &alias_cast, m),
			},
			Match::Or(match_conditions) => write_boolean_group::<_, _, _, false>(
				query,
				context,
				&alias_cast,
				&mut match_conditions.iter().filter(|m| *m != &Match::Any),
			),
		};
		WriteContext::AfterWhereCondition
	}
}

impl WriteWhereClause<&Match<NaiveDateTime>> for Schema
{
	fn write_where_clause(
		context: WriteContext,
		alias: impl Copy + Display,
		match_condition: &Match<NaiveDateTime>,
		query: &mut String,
	) -> WriteContext
	{
		match match_condition
		{
			Match::AllGreaterThan(date) | Match::GreaterThan(date) =>
			{
				write_comparison(query, context, alias, ">", PgTimestampTz(*date))
			},
			Match::AllLessThan(date) | Match::LessThan(date) =>
			{
				write_comparison(query, context, alias, "<", PgTimestampTz(*date))
			},
			Match::AllInRange(low, high) | Match::InRange(low, high) =>
			{
				write_comparison(query, context, alias, "BETWEEN", PgTimestampTz(*low));
				write_comparison(
					query,
					WriteContext::InWhereCondition,
					"",
					"AND",
					PgTimestampTz(*high),
				);
			},
			Match::And(match_conditions) => write_boolean_group::<_, _, _, true>(
				query,
				context,
				alias,
				&mut match_conditions.iter().filter(|m| *m != &Match::Any),
			),
			Match::Any => return context,
			Match::EqualTo(date) => write_comparison(query, context, alias, "=", PgTimestampTz(*date)),
			Match::HasAll(dates) => write_has(
				query,
				context,
				alias,
				dates.iter().copied().map(PgTimestampTz),
				true,
			),
			Match::HasAny(dates) => write_has(
				query,
				context,
				alias,
				dates.iter().copied().map(PgTimestampTz),
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

impl WriteWhereClause<&Match<Option<NaiveDateTime>>> for Schema
{
	fn write_where_clause(
		context: WriteContext,
		alias: impl Copy + Display,
		match_condition: &Match<Option<NaiveDateTime>>,
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
				PgOption(date.map(PgTimestampTz)),
			),
			Match::AllLessThan(date) | Match::LessThan(date) => write_comparison(
				query,
				context,
				alias,
				"<",
				PgOption(date.map(PgTimestampTz)),
			),
			Match::AllInRange(low, high) | Match::InRange(low, high) =>
			{
				write_comparison(
					query,
					context,
					alias,
					"BETWEEN",
					PgOption(low.map(PgTimestampTz)),
				);
				write_comparison(
					query,
					WriteContext::InWhereCondition,
					"",
					"AND",
					PgOption(high.map(PgTimestampTz)),
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
				PgOption(date.map(PgTimestampTz)),
			),
			Match::HasAll(dates) => write_has(
				query,
				context,
				alias,
				dates.iter().map(|o| PgOption(o.map(PgTimestampTz))),
				true,
			),
			Match::HasAny(dates) => write_has(
				query,
				context,
				alias,
				dates.iter().map(|o| PgOption(o.map(PgTimestampTz))),
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

impl WriteWhereClause<&MatchStr<String>> for Schema
{
	/// FIXME: `MatchStr::EqualTo("Foo's Place")` would break this, because of the apostraphe.
	///        Might be able to fix by replacing `'` with `''` before entering.
	fn write_where_clause(
		context: WriteContext,
		alias: impl Copy + Display,
		match_condition: &MatchStr<String>,
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
			MatchStr::Contains(string) =>
			{
				write!(query, "{context} {alias} LIKE '%{string}%'").unwrap()
			},
			MatchStr::EqualTo(string) => write_comparison(query, context, alias, "=", PgStr(string)),
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
			MatchStr::Regex(regex) => write_comparison(query, context, alias, "~", PgStr(regex)),
		};
		WriteContext::AfterWhereCondition
	}
}

impl WriteWhereClause<&MatchEmployee> for Schema
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
				Schema::write_where_clause(context, &format!("{alias}.id"), &match_condition.id, query),
				&format!("{alias}.status"),
				&match_condition.status,
				query,
			),
			&format!("{alias}.title"),
			&match_condition.title,
			query,
		)
	}
}

impl WriteWhereClause<&MatchExpense> for Schema
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
		match_condition: &MatchExpense,
		query: &mut String,
	) -> WriteContext
	{
		Schema::write_where_clause(
			Schema::write_where_clause(
				Schema::write_where_clause(
					Schema::write_where_clause(
						context,
						&format!("{alias}.id"),
						&match_condition.id,
						query,
					),
					&format!("{alias}.category"),
					&match_condition.category,
					query,
				),
				&format!("{alias}.cost"),
				&match_condition.cost,
				query,
			),
			&format!("{alias}.description"),
			&match_condition.description,
			query,
		)
	}
}

impl WriteWhereClause<&MatchJob> for Schema
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
							Schema::write_where_clause(
								Schema::write_where_clause(
									Schema::write_where_clause(
										Schema::write_where_clause(
											context,
											&format!("{alias}.id"),
											&match_condition.id,
											query,
										),
										&format!("{alias}.date_close"),
										&match_condition.date_close,
										query,
									),
									&format!("{alias}.date_open"),
									&match_condition.date_open,
									query,
								),
								&format!("{alias}.increment"),
								&match_condition.increment,
								query,
							),
							&format!("{alias}.invoice_date_issued"),
							&match_condition.invoice.date_issued,
							query,
						),
						&format!("{alias}.invoice_date_paid"),
						&match_condition.invoice.date_paid,
						query,
					),
					&format!("{alias}.invoice_hourly_rate"),
					&match_condition.invoice.hourly_rate,
					query,
				),
				&format!("{alias}.notes"),
				&match_condition.notes,
				query,
			),
			&format!("{alias}.objectives"),
			&match_condition.objectives,
			query,
		)
	}
}

impl WriteWhereClause<&MatchOrganization> for Schema
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
			Schema::write_where_clause(context, &format!("{alias}.id"), &match_condition.id, query),
			&format!("{alias}.name"),
			&match_condition.name,
			query,
		)
	}
}

impl WriteWhereClause<&MatchPerson> for Schema
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
					&format!("{alias}.{}", $column),
					&match_condition.$match_field,
					query,
				)
			};
		}

		write_where_clause!(write_where_clause!(context, "id", id), "name", name)
	}
}

impl WriteWhereClause<&MatchTimesheet> for Schema
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
		match_condition: &MatchTimesheet,
		query: &mut String,
	) -> WriteContext
	{
		Schema::write_where_clause(
			Schema::write_where_clause(
				Schema::write_where_clause(
					context,
					&format!("{alias}.time_begin"),
					&match_condition.time_begin,
					query,
				),
				&format!("{alias}.time_end"),
				&match_condition.time_end,
				query,
			),
			&format!("{alias}.work_notes"),
			&match_condition.work_notes,
			query,
		)
	}
}

#[cfg(test)]
mod tests
{
	use clinvoice_match::{MatchJob, MatchLocation, MatchOuterLocation, MatchTimesheet};
	use clinvoice_schema::chrono::NaiveDate;

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
					Match::Not(Box::new(Match::InRange(0, 10))),
					Match::HasAny(vec![0, 9, 7, 4]),
					Match::Or(vec![
						Match::Not(Box::new(Match::Any)),
						Match::GreaterThan(-1),
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
					&Match::<i64>::AllInRange(0, 2),
					&mut query,
				),
				AfterWhereCondition,
			);
			assert_eq!(
				Schema::write_where_clause(
					InWhereCondition,
					"another_row",
					&Match::<i64>::InRange(0, 2),
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
					&Match::<i64>::AllLessThan(0),
					&mut query,
				),
				AfterWhereCondition,
			);
			assert_eq!(
				Schema::write_where_clause(
					InWhereCondition,
					"another_row",
					&Match::<i64>::LessThan(0),
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
					&Match::<i64>::AllGreaterThan(0),
					&mut query,
				),
				AfterWhereCondition,
			);
			assert_eq!(
				Schema::write_where_clause(
					InWhereCondition,
					"another_row",
					&Match::<i64>::GreaterThan(0),
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
				&MatchStr::Contains("punky brüster".into()),
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
					MatchStr::Regex(r#"^f.rk.*\bit\b.*over$"#.into()),
					MatchStr::Not(Box::new("not equal".into())),
				]),
				&mut query,
			),
			AfterWhereCondition,
		);
		assert_eq!(
			query,
			String::from(
				r#" WHERE ( some_row ~ '^f.rk.*\bit\b.*over$' OR NOT ( some_row = 'not equal'))"#
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
				"P",
				&MatchPerson {
					id: 7.into(),
					..Default::default()
				},
				&mut query,
			),
			AfterWhereCondition
		);
		assert_eq!(query, String::from(" WHERE P.id = 7"));

		query.clear();
		assert_eq!(
			Schema::write_where_clause(
				AfterWhereCondition,
				"P",
				&MatchPerson {
					id: 7.into(),
					name: "stuff".into(),
				},
				&mut query,
			),
			AfterWhereCondition
		);
		assert_eq!(query, String::from(" AND P.id = 7 AND P.name = 'stuff'"));
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
					name: MatchStr::Contains("Gögle".into()),
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

	#[test]
	fn write_job_where_clause()
	{
		let mut query = String::new();
		assert_eq!(
			Schema::write_where_clause(BeforeWhereClause, "J", &MatchJob::default(), &mut query),
			BeforeWhereClause
		);
		assert!(query.is_empty());

		query.clear();
		assert_eq!(
			Schema::write_where_clause(
				BeforeWhereClause,
				"J",
				&MatchJob {
					id: 7.into(),
					date_open: Match::LessThan(NaiveDate::from_ymd(2022, 01, 01).and_hms(12, 37, 22)),
					client: MatchOrganization {
						id: 7.into(),
						name: MatchStr::Contains("Gögle".into()),
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
					..Default::default()
				},
				&mut query,
			),
			AfterWhereCondition
		);
		assert_eq!(
			query,
			String::from(
				" WHERE J.id = 7 AND J.date_open < TIMESTAMP WITH TIME ZONE '2022-01-01 12:37:22'"
			)
		);
	}

	#[test]
	fn write_timesheet_where_clause()
	{
		let mut query = String::new();
		assert_eq!(
			Schema::write_where_clause(BeforeWhereClause, "T", &MatchTimesheet::default(), &mut query),
			BeforeWhereClause
		);
		assert!(query.is_empty());

		query.clear();
	}
}
