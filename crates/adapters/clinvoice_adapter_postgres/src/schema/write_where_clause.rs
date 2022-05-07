use core::{
	fmt::{Display, Write},
	ops::Deref,
	time::Duration,
};

use clinvoice_adapter::{WriteContext, WriteWhereClause};
use clinvoice_finance::Money;
use clinvoice_match::{
	Match,
	MatchContact,
	MatchEmployee,
	MatchExpense,
	MatchJob,
	MatchOrganization,
	MatchPerson,
	MatchSet,
	MatchStr,
	MatchTimesheet,
	Serde,
};
use clinvoice_schema::chrono::NaiveDateTime;

use super::{PgInterval, PgOption, PgSchema as Schema, PgStr, PgTimestampTz};

/// # Summary
///
/// Write multiple `AND`/`OR` `conditions`.
///
/// * If `union` is `true`, the `conditions` are separated by `AND`:
///   `[Match::EqualTo(3), Match::LessThan(4)]` is interpreted as `(foo = 3 AND foo < 4)`.
/// * If `union` is `false`, the `conditions` are separated by `OR`:
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
	conditions: &mut I,
) where
	D: Copy + Display,
	I: Iterator<Item = Q>,
	Schema: WriteWhereClause<Q>,
{
	write!(query, "{context} (").unwrap();
	if let Some(m) = conditions.next()
	{
		Schema::write_where_clause(WriteContext::InWhereCondition, alias, m, query);
	}

	let separator = if UNION { " AND" } else { " OR" };
	conditions.for_each(|q| {
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
			Match::And(conditions) => write_boolean_group::<_, _, _, true>(
				query,
				context,
				alias,
				&mut conditions.iter().filter(|m| *m != &Match::Always),
			),
			Match::Always => return context,
			Match::EqualTo(duration) => write_comparison(
				query,
				context,
				alias,
				"=",
				PgInterval(duration.into_inner()),
			),
			Match::GreaterThan(duration) => write_comparison(
				query,
				context,
				alias,
				">",
				PgInterval(duration.into_inner()),
			),
			Match::InRange(low, high) =>
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
			Match::LessThan(duration) => write_comparison(
				query,
				context,
				alias,
				"<",
				PgInterval(duration.into_inner()),
			),
			Match::Not(condition) => match condition.deref()
			{
				Match::Always => write_is_null(query, context, alias),
				m => write_negated(query, context, alias, m),
			},
			Match::Or(conditions) => write_boolean_group::<_, _, _, false>(
				query,
				context,
				alias,
				&mut conditions.iter().filter(|m| *m != &Match::Always),
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
			Match::And(conditions) => write_boolean_group::<_, _, _, true>(
				query,
				context,
				alias,
				&mut conditions.iter().filter(|m| *m != &Match::Always),
			),
			Match::Always => return context,
			Match::EqualTo(id) => write_comparison(query, context, alias, "=", id),
			Match::GreaterThan(id) => write_comparison(query, context, alias, ">", id),
			Match::InRange(low, high) =>
			{
				write_comparison(query, context, alias, "BETWEEN", low);
				write_comparison(query, WriteContext::InWhereCondition, "", "AND", high);
			},
			Match::LessThan(id) => write_comparison(query, context, alias, "<", id),
			Match::Not(condition) => match condition.deref()
			{
				Match::Always => write_is_null(query, context, alias),
				m => write_negated(query, context, alias, m),
			},
			Match::Or(conditions) => write_boolean_group::<_, _, _, false>(
				query,
				context,
				alias,
				&mut conditions.iter().filter(|m| *m != &Match::Always),
			),
		};
		WriteContext::AfterWhereCondition
	}
}

impl WriteWhereClause<&MatchSet<MatchContact>> for Schema
{
	/// # Panics
	///
	/// If any the following:
	///
	/// * `alias` is an empty string.
	///
	/// # See
	///
	/// * [`WriteWhereClause::write_where_clause`]
	fn write_where_clause(
		context: WriteContext,
		alias: impl Copy + Display,
		match_condition: &MatchSet<MatchContact>,
		query: &mut String,
	) -> WriteContext
	{
		match match_condition {
			MatchSet::Always => return context,

			MatchSet::And(conditions) => write_boolean_group::<_, _, _, true>(
				query,
				context,
				&alias,
				&mut conditions.iter().filter(|m| *m != &MatchSet::Always),
			),

			MatchSet::Contains(match_contact) =>
			{
				let subquery_alias = format!("{alias}_2");
				// NOTE: `{alias}_2` is the name of the dummy table
				write!(query, "{context} EXISTS (SELECT FROM contact_information {subquery_alias} WHERE {subquery_alias}.employee_id = {alias}.employee_id").unwrap();
				Schema::write_where_clause(
					WriteContext::AfterWhereCondition,
					&subquery_alias,
					match_contact,
					query,
				);
				query.push(')');
			},

			MatchSet::Not(condition) => write_negated(query, context, alias, condition.deref()),

			MatchSet::Or(conditions) => write_boolean_group::<_, _, _, false>(
				query,
				context,
				&alias,
				&mut conditions.iter().filter(|m| *m != &MatchSet::Always),
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
			Match::And(conditions) => write_boolean_group::<_, _, _, true>(
				query,
				context,
				&alias_cast,
				&mut conditions.iter().filter(|m| *m != &Match::Always),
			),
			Match::Always => return context,
			Match::EqualTo(money) => write_comparison(query, context, &alias_cast, "=", money),
			Match::GreaterThan(money) =>
			{
				write_comparison(query, context, &alias_cast, ">", money.amount)
			},
			Match::InRange(low, high) =>
			{
				write_comparison(query, context, &alias_cast, "BETWEEN", low);
				write_comparison(query, WriteContext::InWhereCondition, "", "AND", high);
			},
			Match::LessThan(money) => write_comparison(query, context, &alias_cast, "<", money),
			Match::Not(condition) => match condition.deref()
			{
				Match::Always => write_is_null(query, context, &alias_cast),
				m => write_negated(query, context, &alias_cast, m),
			},
			Match::Or(conditions) => write_boolean_group::<_, _, _, false>(
				query,
				context,
				&alias_cast,
				&mut conditions.iter().filter(|m| *m != &Match::Always),
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
			Match::And(conditions) => write_boolean_group::<_, _, _, true>(
				query,
				context,
				alias,
				&mut conditions.iter().filter(|m| *m != &Match::Always),
			),
			Match::Always => return context,
			Match::EqualTo(date) => write_comparison(query, context, alias, "=", PgTimestampTz(*date)),
			Match::GreaterThan(date) =>
			{
				write_comparison(query, context, alias, ">", PgTimestampTz(*date))
			},
			Match::InRange(low, high) =>
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
			Match::LessThan(date) =>
			{
				write_comparison(query, context, alias, "<", PgTimestampTz(*date))
			},
			Match::Not(condition) => match condition.deref()
			{
				Match::Always => write_is_null(query, context, alias),
				m => write_negated(query, context, alias, m),
			},
			Match::Or(conditions) => write_boolean_group::<_, _, _, false>(
				query,
				context,
				alias,
				&mut conditions.iter().filter(|m| *m != &Match::Always),
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
			Match::And(conditions) => write_boolean_group::<_, _, _, true>(
				query,
				context,
				alias,
				&mut conditions.iter().filter(|m| *m != &Match::Always),
			),
			Match::Always => return context,
			Match::EqualTo(date) => write_comparison(
				query,
				context,
				alias,
				"=",
				PgOption(date.map(PgTimestampTz)),
			),
			Match::GreaterThan(date) => write_comparison(
				query,
				context,
				alias,
				">",
				PgOption(date.map(PgTimestampTz)),
			),
			Match::InRange(low, high) =>
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
			Match::LessThan(date) => write_comparison(
				query,
				context,
				alias,
				"<",
				PgOption(date.map(PgTimestampTz)),
			),
			Match::Not(condition) => match condition.deref()
			{
				Match::Always => write_is_null(query, context, alias),
				m => write_negated(query, context, alias, m),
			},
			Match::Or(conditions) => write_boolean_group::<_, _, _, false>(
				query,
				context,
				alias,
				&mut conditions.iter().filter(|m| *m != &Match::Always),
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
			MatchStr::And(conditions) => write_boolean_group::<_, _, _, true>(
				query,
				context,
				alias,
				&mut conditions.iter().filter(|m| *m != &MatchStr::Always),
			),
			MatchStr::Always => return context,
			MatchStr::Contains(string) =>
			{
				write!(query, "{context} {alias} LIKE '%{string}%'").unwrap()
			},
			MatchStr::EqualTo(string) => write_comparison(query, context, alias, "=", PgStr(string)),
			MatchStr::Not(condition) => match condition.deref()
			{
				MatchStr::Always => write_is_null(query, context, alias),
				m => write_negated(query, context, alias, m),
			},
			MatchStr::Or(conditions) => write_boolean_group::<_, _, _, false>(
				query,
				context,
				alias,
				&mut conditions.iter().filter(|m| *m != &MatchStr::Always),
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
