use core::{
	fmt::{Debug, Display, Write},
	ops::Deref,
	time::Duration,
};

use async_recursion::async_recursion;
use clinvoice_adapter::{WriteContext, WriteWhereClause};
use clinvoice_finance::Money;
use clinvoice_match::{
	Match,
	MatchContact,
	MatchContactKind,
	MatchEmployee,
	MatchExpense,
	MatchJob,
	MatchOrganization,
	MatchSet,
	MatchStr,
	MatchTimesheet,
	Serde,
};
use clinvoice_schema::chrono::NaiveDateTime;
use sqlx::{PgPool, Result};

use super::{PgInterval, PgLocation, PgOption, PgSchema as Schema, PgStr, PgTimestampTz};

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

/// # Panics
///
/// If any the following:
///
/// * `alias` is empty.
///
/// # See
///
/// * [`WriteWhereClause::write_where_clause`]
#[async_recursion]
pub(super) async fn write_contact_set_where_clause<A>(
	connection: &PgPool,
	context: WriteContext,
	alias: A,
	match_condition: &MatchSet<MatchContact>,
	query: &mut String,
) -> Result<WriteContext>
where
	A: Copy + Display + Send,
{
	match match_condition
	{
		MatchSet::Any => return Ok(context),

		MatchSet::And(conditions) | MatchSet::Or(conditions) =>
		{
			let iter = &mut conditions.iter().filter(|m| *m != &MatchSet::Any);
			write!(query, "{context} (").unwrap();
			if let Some(c) = iter.next()
			{
				write_contact_set_where_clause(
					connection,
					WriteContext::InWhereCondition,
					alias,
					c,
					query,
				)
				.await?;
			}

			let separator = match match_condition
			{
				MatchSet::And(_) => " AND",
				_ => " OR",
			};

			for c in conditions
			{
				query.push_str(separator);
				write_contact_set_where_clause(
					connection,
					WriteContext::InWhereCondition,
					alias,
					c,
					query,
				)
				.await?;
			}

			query.push(')');
		},

		MatchSet::Contains(match_contact) =>
		{
			let subquery_alias = format!("{alias}_2");
			write!(
				query,
				"{context} EXISTS (
				SELECT FROM contact_information {subquery_alias}
				WHERE {subquery_alias}.organization_id = {alias}.organization_id"
			)
			.unwrap();

			let ctx = Schema::write_where_clause(
				Schema::write_where_clause(
					WriteContext::AfterWhereCondition,
					&format!("{alias}.label"),
					&match_contact.label,
					query,
				),
				&format!("{alias}.export"),
				&match_contact.export,
				query,
			);

			match match_contact.kind
			{
				MatchContactKind::Always => (),

				MatchContactKind::SomeAddress(ref location) =>
				{
					let location_id_query =
						PgLocation::retrieve_matching_ids(connection, location).await?;

					Schema::write_where_clause(
						ctx,
						&format!("{alias}.address_id"),
						&location_id_query,
						query,
					);
				},

				MatchContactKind::SomeEmail(ref email_address) =>
				{
					Schema::write_where_clause(ctx, &format!("{alias}.email"), email_address, query);
				},

				MatchContactKind::SomePhone(ref phone_number) =>
				{
					Schema::write_where_clause(ctx, &format!("{alias}.phone"), phone_number, query);
				},
			};

			query.push(')');
		},

		MatchSet::Not(condition) =>
		{
			write!(query, "{context} NOT (").unwrap();
			write_contact_set_where_clause(
				connection,
				WriteContext::InWhereCondition,
				alias,
				condition,
				query,
			)
			.await?;
			query.push(')');
		},
	};

	Ok(WriteContext::AfterWhereCondition)
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

/// # Summary
///
/// A blanket implementation of [`WriteWhereClause`].
///
/// Requires access to something else that has already implemented [`WriteWhereClause`] for
/// [`Schema`], so that methods like [`write_boolean_group`] can be abstracted away from _this_
/// implementation.
fn write_where_clause<T>(
	context: WriteContext,
	alias: impl Copy + Display,
	match_condition: &Match<T>,
	query: &mut String,
) -> WriteContext
where
	T: Clone + Debug + Display + PartialEq,
	for<'a> Schema: WriteWhereClause<&'a Match<T>>,
{
	match match_condition
	{
		Match::And(conditions) => write_boolean_group::<_, _, _, true>(
			query,
			context,
			alias,
			&mut conditions.iter().filter(|m| *m != &Match::Any),
		),
		Match::Any => return context,
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
			Match::Any => write_is_null(query, context, alias),
			m => write_negated(query, context, alias, m),
		},
		Match::Or(conditions) => write_boolean_group::<_, _, _, false>(
			query,
			context,
			alias,
			&mut conditions.iter().filter(|m| *m != &Match::Any),
		),
	};
	WriteContext::AfterWhereCondition
}

impl WriteWhereClause<&Match<bool>> for Schema
{
	fn write_where_clause(
		context: WriteContext,
		alias: impl Copy + Display,
		match_condition: &Match<bool>,
		query: &mut String,
	) -> WriteContext
	{
		write_where_clause(context, alias, match_condition, query)
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
		write_where_clause(context, alias, match_condition, query)
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
		write_where_clause(
			context,
			&format!("{alias}::numeric"),
			match_condition,
			query,
		)
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
				&mut conditions.iter().filter(|m| *m != &Match::Any),
			),
			Match::Any => return context,
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
				Match::Any => write_is_null(query, context, alias),
				m => write_negated(query, context, alias, m),
			},
			Match::Or(conditions) => write_boolean_group::<_, _, _, false>(
				query,
				context,
				alias,
				&mut conditions.iter().filter(|m| *m != &Match::Any),
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
				&mut conditions.iter().filter(|m| *m != &Match::Any),
			),
			Match::Any => return context,
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
				Match::Any => write_is_null(query, context, alias),
				m => write_negated(query, context, alias, m),
			},
			Match::Or(conditions) => write_boolean_group::<_, _, _, false>(
				query,
				context,
				alias,
				&mut conditions.iter().filter(|m| *m != &Match::Any),
			),
		};
		WriteContext::AfterWhereCondition
	}
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
				&mut conditions.iter().filter(|m| *m != &Match::Any),
			),
			Match::Any => return context,
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
				Match::Any => write_is_null(query, context, alias),
				m => write_negated(query, context, alias, m),
			},
			Match::Or(conditions) => write_boolean_group::<_, _, _, false>(
				query,
				context,
				alias,
				&mut conditions.iter().filter(|m| *m != &Match::Any),
			),
		};
		WriteContext::AfterWhereCondition
	}
}

impl WriteWhereClause<&MatchSet<MatchExpense>> for Schema
{
	fn write_where_clause(
		context: WriteContext,
		alias: impl Copy + Display,
		match_condition: &MatchSet<MatchExpense>,
		query: &mut String,
	) -> WriteContext
	{
		match match_condition
		{
			MatchSet::Any => return context,

			MatchSet::And(conditions) | MatchSet::Or(conditions) =>
			{
				let iter = &mut conditions.iter().filter(|m| *m != &MatchSet::Any);
				write!(query, "{context} (").unwrap();
				if let Some(c) = iter.next()
				{
					Schema::write_where_clause(WriteContext::InWhereCondition, alias, c, query);
				}

				let separator = match match_condition
				{
					MatchSet::And(_) => " AND",
					_ => " OR",
				};

				for c in conditions
				{
					query.push_str(separator);
					Schema::write_where_clause(WriteContext::InWhereCondition, alias, c, query);
				}

				query.push(')');
			},

			MatchSet::Contains(match_expense) =>
			{
				let subquery_alias = format!("{alias}_2");
				write!(
					query,
					"{context} EXISTS (
					SELECT FROM expenses {subquery_alias}
					WHERE {subquery_alias}.id = {alias}.id"
				)
				.unwrap();

				Schema::write_where_clause(
					WriteContext::AfterWhereCondition,
					&subquery_alias,
					match_expense,
					query,
				);

				query.push(')');
			},

			MatchSet::Not(condition) =>
			{
				write!(query, "{context} NOT (").unwrap();
				Schema::write_where_clause(
					WriteContext::InWhereCondition,
					alias,
					condition.deref(),
					query,
				);
				query.push(')');
			},
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
				&mut conditions.iter().filter(|m| *m != &MatchStr::Any),
			),
			MatchStr::Any => return context,
			MatchStr::Contains(string) =>
			{
				write!(query, "{context} {alias} LIKE '%{string}%'").unwrap()
			},
			MatchStr::EqualTo(string) => write_comparison(query, context, alias, "=", PgStr(string)),
			MatchStr::Not(condition) => match condition.deref()
			{
				MatchStr::Any => write_is_null(query, context, alias),
				m => write_negated(query, context, alias, m),
			},
			MatchStr::Or(conditions) => write_boolean_group::<_, _, _, false>(
				query,
				context,
				alias,
				&mut conditions.iter().filter(|m| *m != &MatchStr::Any),
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
				Schema::write_where_clause(
					Schema::write_where_clause(
						context,
						&format!("{alias}.id"),
						&match_condition.id,
						query,
					),
					&format!("{alias}.name"),
					&match_condition.name,
					query,
				),
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
					Schema::write_where_clause(
						context,
						&format!("{alias}.id"),
						&match_condition.id,
						query,
					),
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
