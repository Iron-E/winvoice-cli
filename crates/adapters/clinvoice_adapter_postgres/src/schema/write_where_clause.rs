use core::{
	fmt::{Debug, Display},
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
use sqlx::{Database, PgPool, Postgres, QueryBuilder, Result};

use super::{PgInterval, PgLocation, PgOption, PgSchema as Schema, PgTimestampTz};

/// # Summary
///
/// Append `"{context} ("` to `query`. If `NOT` is `true`, then everything preceding a
/// closing [`write_context_scope_end`] is negated.
fn write_context_scope_start<Db, const NEGATE: bool>(
	query: &mut QueryBuilder<Db>,
	context: WriteContext,
) where
	Db: Database,
{
	let mut separated = query.separated(' ');
	separated.push(context);
	if NEGATE
	{
		separated.push("NOT");
	}
	separated.push('(');
}

/// # Summary
///
/// Write `')'` to the `query`, ending some prior [`write_context_scope_start`].
fn write_context_scope_end<Db>(query: &mut QueryBuilder<Db>)
where
	Db: Database,
{
	query.push(')');
}

/// # Summary
///
/// Write multiple `AND`/`OR` `conditions`.
///
/// * If `union` is `true`, the `conditions` are separated by `AND`:
///   `[Match::EqualTo(3), Match::LessThan(4)]` is interpreted as `(foo = 3 AND foo < 4)`.
/// * If `union` is `false`, the `conditions` are separated by `OR`:
///   `[Match::EqualTo(3), Match::LessThan(4)]` is interpreted as `(foo = 3 OR foo < 4)`.
///
/// The rest of the args are the same as [`WriteSql::write_where`].
fn write_boolean_group<D, Db, I, M, const UNION: bool>(
	query: &mut QueryBuilder<Db>,
	context: WriteContext,
	alias: D,
	conditions: &mut I,
) where
	D: Copy + Display,
	Db: Database,
	I: Iterator<Item = M>,
	Schema: WriteWhereClause<Db, M>,
{
	write_context_scope_start::<_, false>(query, context);

	if let Some(m) = conditions.next()
	{
		Schema::write_where_clause(WriteContext::InWhereCondition, alias, m, query);
	}

	let separator = if UNION { " AND" } else { " OR" };
	conditions.for_each(|c| {
		query.push(separator);
		Schema::write_where_clause(WriteContext::InWhereCondition, alias, c, query);
	});

	write_context_scope_end(query);
}

/// # Summary
///
/// Write a comparison of `alias` and `comparand` using `comparator`.
///
/// The rest of the args are the same as [`WriteSql::write_where`].
///
/// # Warnings
///
/// Does not guard against SQL injection.
fn write_comparison<Db>(
	query: &mut QueryBuilder<Db>,
	context: WriteContext,
	alias: impl Copy + Display,
	comparator: &str,
	comparand: impl Copy + Display,
) where
	Db: Database,
{
	query
		.separated(' ')
		.push(context)
		.push(alias)
		.push(comparator)
		.push(comparand);
}

/// # Warnings
///
/// Does not guard against SQL injection.
///
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
	query: &mut QueryBuilder<Postgres>,
) -> Result<WriteContext>
where
	A: Copy + Display + Send,
{
	match match_condition
	{
		MatchSet::Any => return Ok(context),

		MatchSet::And(conditions) | MatchSet::Or(conditions) =>
		{
			write_context_scope_start::<_, false>(query, context);

			let iter = &mut conditions.iter().filter(|m| *m != &MatchSet::Any);
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
				query.push(separator);
				write_contact_set_where_clause(
					connection,
					WriteContext::InWhereCondition,
					alias,
					c,
					query,
				)
				.await?;
			}

			write_context_scope_end(query);
		},

		MatchSet::Contains(match_contact) =>
		{
			let subquery_alias = format!("{alias}_2");

			query
				.separated(' ')
				.push(context)
				.push("EXISTS (SELECT FROM contact_information")
				.push(&subquery_alias)
				.push("WHERE ");
			query
				.push(&subquery_alias)
				.push(".organization_id = ")
				.push(alias)
				.push(".organization_id");

			let ctx = Schema::write_where_clause(
				Schema::write_where_clause(
					WriteContext::AcceptingAnotherWhereCondition,
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
			write_context_scope_start::<_, true>(query, context);

			write_contact_set_where_clause(
				connection,
				WriteContext::InWhereCondition,
				alias,
				condition,
				query,
			)
			.await?;

			write_context_scope_end(query);
		},
	};

	Ok(WriteContext::AcceptingAnotherWhereCondition)
}

/// # Summary
///
/// Write a comparison of `alias` and `comparand` using `comparator`.
///
/// The rest of the args are the same as [`WriteSql::write_where`].
///
/// # Warnings
///
/// Does not guard against SQL injection.
fn write_is_null<Db>(
	query: &mut QueryBuilder<Db>,
	context: WriteContext,
	alias: impl Copy + Display,
) where
	Db: Database,
{
	query
		.separated(' ')
		.push(context)
		.push(alias)
		.push("IS NULL");
}

/// # Summary
///
/// Wrap some `match_condition` in `NOT (…)`.
///
/// The args are the same as [`WriteSql::write_where`].
///
/// # Warnings
///
/// Does not guard against SQL injection.
fn write_negated<Db, M>(
	query: &mut QueryBuilder<Db>,
	context: WriteContext,
	alias: impl Copy + Display,
	match_condition: M,
) where
	Db: Database,
	Schema: WriteWhereClause<Db, M>,
{
	write_context_scope_start::<_, true>(query, context);

	Schema::write_where_clause(
		WriteContext::InWhereCondition,
		alias,
		match_condition,
		query,
	);

	write_context_scope_end(query);
}

/// # Summary
///
/// A blanket implementation of [`WriteWhereClause`].
///
/// Requires access to something else that has already implemented [`WriteWhereClause`] for
/// [`Schema`], so that methods like [`write_boolean_group`] can be abstracted away from _this_
/// implementation.
///
/// # Warnings
///
/// Does not guard against SQL injection.
fn write_where_clause<Db, T>(
	context: WriteContext,
	alias: impl Copy + Display,
	match_condition: &Match<T>,
	query: &mut QueryBuilder<Db>,
) -> WriteContext
where
	Db: Database,
	T: Clone + Debug + Display + PartialEq,
	for<'a> Schema: WriteWhereClause<Db, &'a Match<T>>,
{
	match match_condition
	{
		Match::And(conditions) => write_boolean_group::<_, _, _, _, true>(
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
		Match::Or(conditions) => write_boolean_group::<_, _, _, _, false>(
			query,
			context,
			alias,
			&mut conditions.iter().filter(|m| *m != &Match::Any),
		),
	};
	WriteContext::AcceptingAnotherWhereCondition
}

impl WriteWhereClause<Postgres, &Match<bool>> for Schema
{
	fn write_where_clause(
		context: WriteContext,
		alias: impl Copy + Display,
		match_condition: &Match<bool>,
		query: &mut QueryBuilder<Postgres>,
	) -> WriteContext
	{
		write_where_clause(context, alias, match_condition, query)
	}
}

impl WriteWhereClause<Postgres, &Match<i64>> for Schema
{
	fn write_where_clause(
		context: WriteContext,
		alias: impl Copy + Display,
		match_condition: &Match<i64>,
		query: &mut QueryBuilder<Postgres>,
	) -> WriteContext
	{
		write_where_clause(context, alias, match_condition, query)
	}
}

impl WriteWhereClause<Postgres, &Match<Money>> for Schema
{
	fn write_where_clause(
		context: WriteContext,
		alias: impl Copy + Display,
		match_condition: &Match<Money>,
		query: &mut QueryBuilder<Postgres>,
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

impl WriteWhereClause<Postgres, &Match<NaiveDateTime>> for Schema
{
	fn write_where_clause(
		context: WriteContext,
		alias: impl Copy + Display,
		match_condition: &Match<NaiveDateTime>,
		query: &mut QueryBuilder<Postgres>,
	) -> WriteContext
	{
		match match_condition
		{
			Match::And(conditions) => write_boolean_group::<_, _, _, _, true>(
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
			Match::Or(conditions) => write_boolean_group::<_, _, _, _, false>(
				query,
				context,
				alias,
				&mut conditions.iter().filter(|m| *m != &Match::Any),
			),
		};
		WriteContext::AcceptingAnotherWhereCondition
	}
}

impl WriteWhereClause<Postgres, &Match<Option<NaiveDateTime>>> for Schema
{
	fn write_where_clause(
		context: WriteContext,
		alias: impl Copy + Display,
		match_condition: &Match<Option<NaiveDateTime>>,
		query: &mut QueryBuilder<Postgres>,
	) -> WriteContext
	{
		match match_condition
		{
			Match::And(conditions) => write_boolean_group::<_, _, _, _, true>(
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
			Match::Or(conditions) => write_boolean_group::<_, _, _, _, false>(
				query,
				context,
				alias,
				&mut conditions.iter().filter(|m| *m != &Match::Any),
			),
		};
		WriteContext::AcceptingAnotherWhereCondition
	}
}

impl WriteWhereClause<Postgres, &Match<Serde<Duration>>> for Schema
{
	fn write_where_clause(
		context: WriteContext,
		alias: impl Copy + Display,
		match_condition: &Match<Serde<Duration>>,
		query: &mut QueryBuilder<Postgres>,
	) -> WriteContext
	{
		match match_condition
		{
			Match::And(conditions) => write_boolean_group::<_, _, _, _, true>(
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
			Match::Or(conditions) => write_boolean_group::<_, _, _, _, false>(
				query,
				context,
				alias,
				&mut conditions.iter().filter(|m| *m != &Match::Any),
			),
		};
		WriteContext::AcceptingAnotherWhereCondition
	}
}

impl WriteWhereClause<Postgres, &MatchSet<MatchExpense>> for Schema
{
	fn write_where_clause(
		context: WriteContext,
		alias: impl Copy + Display,
		match_condition: &MatchSet<MatchExpense>,
		query: &mut QueryBuilder<Postgres>,
	) -> WriteContext
	{
		match match_condition
		{
			MatchSet::Any => return context,

			MatchSet::And(conditions) | MatchSet::Or(conditions) =>
			{
				write_context_scope_start::<_, false>(query, context);

				let iter = &mut conditions.iter().filter(|m| *m != &MatchSet::Any);
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
					query.push(separator);
					Schema::write_where_clause(WriteContext::InWhereCondition, alias, c, query);
				}

				write_context_scope_end(query);
			},

			MatchSet::Contains(match_expense) =>
			{
				let subquery_alias = format!("{alias}_2");

				query
					.separated(' ')
					.push(context)
					.push("EXISTS (SELECT FROM expenses")
					.push(&subquery_alias)
					.push("WHERE ");
				query
					.push(&subquery_alias)
					.push(".timesheet_id = ")
					.push(alias)
					.push(".timesheet_id");

				Schema::write_where_clause(
					WriteContext::AcceptingAnotherWhereCondition,
					&subquery_alias,
					match_expense,
					query,
				);

				query.push(')');
			},

			MatchSet::Not(condition) =>
			{
				write_context_scope_start::<_, true>(query, context);

				Schema::write_where_clause(
					WriteContext::InWhereCondition,
					alias,
					condition.deref(),
					query,
				);

				write_context_scope_end(query);
			},
		};

		WriteContext::AcceptingAnotherWhereCondition
	}
}

impl WriteWhereClause<Postgres, &MatchStr<String>> for Schema
{
	fn write_where_clause(
		context: WriteContext,
		alias: impl Copy + Display,
		match_condition: &MatchStr<String>,
		query: &mut QueryBuilder<Postgres>,
	) -> WriteContext
	{
		// NOTE: we cannot use certain helpers defined above, as some do not
		// sanitize `match_condition` and are thus susceptible to SQL injection.
		match match_condition
		{
			MatchStr::And(conditions) => write_boolean_group::<_, _, _, _, true>(
				query,
				context,
				alias,
				&mut conditions.iter().filter(|m| *m != &MatchStr::Any),
			),
			MatchStr::Any => return context,
			MatchStr::Contains(string) =>
			{
				query
					.separated(' ')
					.push(context)
					.push(alias)
					.push("LIKE")
					// HACK: this is the only way I could think to surround `string` with the syntax
					//       needed (e.g. `foo LIKE '%o%'`) and still sanitize it.
					.push_bind(format!("%{string}%"));
			},
			MatchStr::EqualTo(string) =>
			{
				query
					.separated(' ')
					.push(context)
					.push('=')
					.push_bind(string.clone());
			},
			MatchStr::Not(condition) => match condition.deref()
			{
				MatchStr::Any => write_is_null(query, context, alias),
				m => write_negated(query, context, alias, m),
			},
			MatchStr::Or(conditions) => write_boolean_group::<_, _, _, _, false>(
				query,
				context,
				alias,
				&mut conditions.iter().filter(|m| *m != &MatchStr::Any),
			),
			MatchStr::Regex(regex) =>
			{
				query
					.separated(' ')
					.push(context)
					.push('~')
					.push_bind(regex.clone());
			},
		};
		WriteContext::AcceptingAnotherWhereCondition
	}
}

impl WriteWhereClause<Postgres, &MatchEmployee> for Schema
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
		query: &mut QueryBuilder<Postgres>,
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

impl WriteWhereClause<Postgres, &MatchExpense> for Schema
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
		query: &mut QueryBuilder<Postgres>,
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

impl WriteWhereClause<Postgres, &MatchJob> for Schema
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
		query: &mut QueryBuilder<Postgres>,
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

impl WriteWhereClause<Postgres, &MatchOrganization> for Schema
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
		query: &mut QueryBuilder<Postgres>,
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

impl WriteWhereClause<Postgres, &MatchTimesheet> for Schema
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
		query: &mut QueryBuilder<Postgres>,
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
