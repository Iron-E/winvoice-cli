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
use clinvoice_schema::{chrono::NaiveDateTime, Id};
use sqlx::{Database, PgPool, Postgres, QueryBuilder, Result};

use super::{PgInterval, PgLocation, PgOption, PgSchema, PgTimestampTz, organization::columns::PgOrganizationColumns};
use crate::schema::{
	contact_info::columns::PgContactColumns,
	employee::columns::PgEmployeeColumns,
	expenses::columns::PgExpenseColumns,
	job::columns::PgJobColumns,
	timesheet::columns::PgTimesheetColumns,
};

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
	ident: D,
	conditions: &mut I,
) where
	D: Copy + Display,
	Db: Database,
	I: Iterator<Item = M>,
	PgSchema: WriteWhereClause<Db, M>,
{
	write_context_scope_start::<_, false>(query, context);

	if let Some(m) = conditions.next()
	{
		PgSchema::write_where_clause(WriteContext::InWhereCondition, ident, m, query);
	}

	let separator = if UNION { " AND" } else { " OR" };
	conditions.for_each(|c| {
		query.push(separator);
		PgSchema::write_where_clause(WriteContext::InWhereCondition, ident, c, query);
	});

	write_context_scope_end(query);
}

/// # Summary
///
/// Write a comparison of `ident` and `comparand` using `comparator`.
///
/// The rest of the args are the same as [`WriteSql::write_where`].
///
/// # Warnings
///
/// Does not guard against SQL injection.
fn write_comparison<Db>(
	query: &mut QueryBuilder<Db>,
	context: WriteContext,
	ident: impl Copy + Display,
	comparator: &str,
	comparand: impl Copy + Display,
) where
	Db: Database,
{
	query
		.separated(' ')
		.push(context)
		.push(ident)
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
/// * `ident` is empty.
///
/// # See
///
/// * [`WriteWhereClause::write_where_clause`]
#[async_recursion]
pub(super) async fn write_contact_set_where_clause<A>(
	connection: &PgPool,
	context: WriteContext,
	ident: A,
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
					ident,
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
					ident,
					c,
					query,
				)
				.await?;
			}

			write_context_scope_end(query);
		},

		MatchSet::Contains(match_contact) =>
		{
			let subquery_ident = format!("{ident}_2");

			const COLUMNS: PgContactColumns<&'static str> = PgContactColumns::new();
			let alias_columns = COLUMNS.scoped(ident);
			let subquery_columns = COLUMNS.scoped(&subquery_ident);

			query
				.separated(' ')
				.push(context)
				.push("EXISTS (SELECT FROM contact_information")
				.push(&subquery_ident)
				.push("WHERE")
				.push(subquery_columns.organization_id)
				.push('=')
				.push(alias_columns.organization_id);

			let ctx = PgSchema::write_where_clause(
				PgSchema::write_where_clause(
					WriteContext::AcceptingAnotherWhereCondition,
					alias_columns.label,
					&match_contact.label,
					query,
				),
				alias_columns.export,
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

					PgSchema::write_where_clause(
						ctx,
						alias_columns.address_id,
						&location_id_query,
						query,
					);
				},

				MatchContactKind::SomeEmail(ref email_address) =>
				{
					PgSchema::write_where_clause(ctx, alias_columns.email, email_address, query);
				},

				MatchContactKind::SomePhone(ref phone_number) =>
				{
					PgSchema::write_where_clause(ctx, alias_columns.phone, phone_number, query);
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
				ident,
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
/// Write a comparison of `ident` and `comparand` using `comparator`.
///
/// The rest of the args are the same as [`WriteSql::write_where`].
///
/// # Warnings
///
/// Does not guard against SQL injection.
fn write_is_null<Db>(
	query: &mut QueryBuilder<Db>,
	context: WriteContext,
	ident: impl Copy + Display,
) where
	Db: Database,
{
	query
		.separated(' ')
		.push(context)
		.push(ident)
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
	ident: impl Copy + Display,
	match_condition: M,
) where
	Db: Database,
	PgSchema: WriteWhereClause<Db, M>,
{
	write_context_scope_start::<_, true>(query, context);

	PgSchema::write_where_clause(
		WriteContext::InWhereCondition,
		ident,
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
/// [`PgSchema`], so that methods like [`write_boolean_group`] can be abstracted away from _this_
/// implementation.
///
/// # Warnings
///
/// Does not guard against SQL injection.
fn write_where_clause<Db, T>(
	context: WriteContext,
	ident: impl Copy + Display,
	match_condition: &Match<T>,
	query: &mut QueryBuilder<Db>,
) -> WriteContext
where
	Db: Database,
	T: Clone + Debug + Display + PartialEq,
	for<'a> PgSchema: WriteWhereClause<Db, &'a Match<T>>,
{
	match match_condition
	{
		Match::And(conditions) => write_boolean_group::<_, _, _, _, true>(
			query,
			context,
			ident,
			&mut conditions.iter().filter(|m| *m != &Match::Any),
		),
		Match::Any => return context,
		Match::EqualTo(id) => write_comparison(query, context, ident, "=", id),
		Match::GreaterThan(id) => write_comparison(query, context, ident, ">", id),
		Match::InRange(low, high) =>
		{
			write_comparison(query, context, ident, "BETWEEN", low);
			write_comparison(query, WriteContext::InWhereCondition, "", "AND", high);
		},
		Match::LessThan(id) => write_comparison(query, context, ident, "<", id),
		Match::Not(condition) => match condition.deref()
		{
			Match::Any => write_is_null(query, context, ident),
			m => write_negated(query, context, ident, m),
		},
		Match::Or(conditions) => write_boolean_group::<_, _, _, _, false>(
			query,
			context,
			ident,
			&mut conditions.iter().filter(|m| *m != &Match::Any),
		),
	};
	WriteContext::AcceptingAnotherWhereCondition
}

impl WriteWhereClause<Postgres, &Match<bool>> for PgSchema
{
	fn write_where_clause(
		context: WriteContext,
		ident: impl Copy + Display,
		match_condition: &Match<bool>,
		query: &mut QueryBuilder<Postgres>,
	) -> WriteContext
	{
		write_where_clause(context, ident, match_condition, query)
	}
}

impl WriteWhereClause<Postgres, &Match<Id>> for PgSchema
{
	fn write_where_clause(
		context: WriteContext,
		ident: impl Copy + Display,
		match_condition: &Match<Id>,
		query: &mut QueryBuilder<Postgres>,
	) -> WriteContext
	{
		write_where_clause(context, ident, match_condition, query)
	}
}

impl WriteWhereClause<Postgres, &Match<Money>> for PgSchema
{
	fn write_where_clause(
		context: WriteContext,
		ident: impl Copy + Display,
		match_condition: &Match<Money>,
		query: &mut QueryBuilder<Postgres>,
	) -> WriteContext
	{
		// TODO: use `PgTypeCast::numeric(ident)` after rust-lang/rust#39959
		write_where_clause(
			context,
			&format!("{ident}::numeric"),
			match_condition,
			query,
		)
	}
}

impl WriteWhereClause<Postgres, &Match<NaiveDateTime>> for PgSchema
{
	fn write_where_clause(
		context: WriteContext,
		ident: impl Copy + Display,
		match_condition: &Match<NaiveDateTime>,
		query: &mut QueryBuilder<Postgres>,
	) -> WriteContext
	{
		match match_condition
		{
			Match::And(conditions) => write_boolean_group::<_, _, _, _, true>(
				query,
				context,
				ident,
				&mut conditions.iter().filter(|m| *m != &Match::Any),
			),
			Match::Any => return context,
			Match::EqualTo(date) => write_comparison(query, context, ident, "=", PgTimestampTz(*date)),
			Match::GreaterThan(date) =>
			{
				write_comparison(query, context, ident, ">", PgTimestampTz(*date))
			},
			Match::InRange(low, high) =>
			{
				write_comparison(query, context, ident, "BETWEEN", PgTimestampTz(*low));
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
				write_comparison(query, context, ident, "<", PgTimestampTz(*date))
			},
			Match::Not(condition) => match condition.deref()
			{
				Match::Any => write_is_null(query, context, ident),
				m => write_negated(query, context, ident, m),
			},
			Match::Or(conditions) => write_boolean_group::<_, _, _, _, false>(
				query,
				context,
				ident,
				&mut conditions.iter().filter(|m| *m != &Match::Any),
			),
		};
		WriteContext::AcceptingAnotherWhereCondition
	}
}

impl WriteWhereClause<Postgres, &Match<Option<NaiveDateTime>>> for PgSchema
{
	fn write_where_clause(
		context: WriteContext,
		ident: impl Copy + Display,
		match_condition: &Match<Option<NaiveDateTime>>,
		query: &mut QueryBuilder<Postgres>,
	) -> WriteContext
	{
		match match_condition
		{
			Match::And(conditions) => write_boolean_group::<_, _, _, _, true>(
				query,
				context,
				ident,
				&mut conditions.iter().filter(|m| *m != &Match::Any),
			),
			Match::Any => return context,
			Match::EqualTo(date) => write_comparison(
				query,
				context,
				ident,
				"=",
				PgOption(date.map(PgTimestampTz)),
			),
			Match::GreaterThan(date) => write_comparison(
				query,
				context,
				ident,
				">",
				PgOption(date.map(PgTimestampTz)),
			),
			Match::InRange(low, high) =>
			{
				write_comparison(
					query,
					context,
					ident,
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
				ident,
				"<",
				PgOption(date.map(PgTimestampTz)),
			),
			Match::Not(condition) => match condition.deref()
			{
				Match::Any => write_is_null(query, context, ident),
				m => write_negated(query, context, ident, m),
			},
			Match::Or(conditions) => write_boolean_group::<_, _, _, _, false>(
				query,
				context,
				ident,
				&mut conditions.iter().filter(|m| *m != &Match::Any),
			),
		};
		WriteContext::AcceptingAnotherWhereCondition
	}
}

impl WriteWhereClause<Postgres, &Match<Serde<Duration>>> for PgSchema
{
	fn write_where_clause(
		context: WriteContext,
		ident: impl Copy + Display,
		match_condition: &Match<Serde<Duration>>,
		query: &mut QueryBuilder<Postgres>,
	) -> WriteContext
	{
		match match_condition
		{
			Match::And(conditions) => write_boolean_group::<_, _, _, _, true>(
				query,
				context,
				ident,
				&mut conditions.iter().filter(|m| *m != &Match::Any),
			),
			Match::Any => return context,
			Match::EqualTo(duration) => write_comparison(
				query,
				context,
				ident,
				"=",
				PgInterval(duration.into_inner()),
			),
			Match::GreaterThan(duration) => write_comparison(
				query,
				context,
				ident,
				">",
				PgInterval(duration.into_inner()),
			),
			Match::InRange(low, high) =>
			{
				write_comparison(
					query,
					context,
					ident,
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
				ident,
				"<",
				PgInterval(duration.into_inner()),
			),
			Match::Not(condition) => match condition.deref()
			{
				Match::Any => write_is_null(query, context, ident),
				m => write_negated(query, context, ident, m),
			},
			Match::Or(conditions) => write_boolean_group::<_, _, _, _, false>(
				query,
				context,
				ident,
				&mut conditions.iter().filter(|m| *m != &Match::Any),
			),
		};
		WriteContext::AcceptingAnotherWhereCondition
	}
}

impl WriteWhereClause<Postgres, &MatchSet<MatchExpense>> for PgSchema
{
	fn write_where_clause(
		context: WriteContext,
		ident: impl Copy + Display,
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
					PgSchema::write_where_clause(WriteContext::InWhereCondition, ident, c, query);
				}

				let separator = match match_condition
				{
					MatchSet::And(_) => " AND",
					_ => " OR",
				};

				for c in conditions
				{
					query.push(separator);
					PgSchema::write_where_clause(WriteContext::InWhereCondition, ident, c, query);
				}

				write_context_scope_end(query);
			},

			MatchSet::Contains(match_expense) =>
			{
				let subquery_ident = format!("{ident}_2");

				query
					.separated(' ')
					.push(context)
					.push("EXISTS (SELECT FROM expenses")
					.push(&subquery_ident)
					.push("WHERE ");
				query
					.push(&subquery_ident)
					.push(".timesheet_id = ")
					.push(ident)
					.push(".timesheet_id");

				PgSchema::write_where_clause(
					WriteContext::AcceptingAnotherWhereCondition,
					&subquery_ident,
					match_expense,
					query,
				);

				query.push(')');
			},

			MatchSet::Not(condition) =>
			{
				write_context_scope_start::<_, true>(query, context);

				PgSchema::write_where_clause(
					WriteContext::InWhereCondition,
					ident,
					condition.deref(),
					query,
				);

				write_context_scope_end(query);
			},
		};

		WriteContext::AcceptingAnotherWhereCondition
	}
}

impl WriteWhereClause<Postgres, &MatchStr<String>> for PgSchema
{
	fn write_where_clause(
		context: WriteContext,
		ident: impl Copy + Display,
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
				ident,
				&mut conditions.iter().filter(|m| *m != &MatchStr::Any),
			),
			MatchStr::Any => return context,
			MatchStr::Contains(string) =>
			{
				query
					.separated(' ')
					.push(context)
					.push(ident)
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
					.push(ident)
					.push('=')
					.push_bind(string.clone());
			},
			MatchStr::Not(condition) => match condition.deref()
			{
				MatchStr::Any => write_is_null(query, context, ident),
				m => write_negated(query, context, ident, m),
			},
			MatchStr::Or(conditions) => write_boolean_group::<_, _, _, _, false>(
				query,
				context,
				ident,
				&mut conditions.iter().filter(|m| *m != &MatchStr::Any),
			),
			MatchStr::Regex(regex) =>
			{
				query
					.separated(' ')
					.push(context)
					.push(ident)
					.push('~')
					.push_bind(regex.clone());
			},
		};
		WriteContext::AcceptingAnotherWhereCondition
	}
}

impl WriteWhereClause<Postgres, &MatchEmployee> for PgSchema
{
	/// # Panics
	///
	/// If any the following:
	///
	/// * `ident` is an empty string.
	///
	/// # See
	///
	/// * [`WriteWhereClause::write_where_clause`]
	fn write_where_clause(
		context: WriteContext,
		ident: impl Copy + Display,
		match_condition: &MatchEmployee,
		query: &mut QueryBuilder<Postgres>,
	) -> WriteContext
	{
		let columns = PgEmployeeColumns::new().scoped(ident);

		PgSchema::write_where_clause(
			PgSchema::write_where_clause(
				PgSchema::write_where_clause(
					PgSchema::write_where_clause(context, columns.id, &match_condition.id, query),
					columns.name,
					&match_condition.name,
					query,
				),
				columns.status,
				&match_condition.status,
				query,
			),
			columns.title,
			&match_condition.title,
			query,
		)
	}
}

impl WriteWhereClause<Postgres, &MatchExpense> for PgSchema
{
	/// # Panics
	///
	/// If any the following:
	///
	/// * `ident` is an empty string.
	///
	/// # See
	///
	/// * [`WriteWhereClause::write_where_clause`]
	fn write_where_clause(
		context: WriteContext,
		ident: impl Copy + Display,
		match_condition: &MatchExpense,
		query: &mut QueryBuilder<Postgres>,
	) -> WriteContext
	{
		let columns = PgExpenseColumns::new().scoped(ident);

		PgSchema::write_where_clause(
			PgSchema::write_where_clause(
				PgSchema::write_where_clause(
					PgSchema::write_where_clause(context, columns.id, &match_condition.id, query),
					columns.category,
					&match_condition.category,
					query,
				),
				columns.cost,
				&match_condition.cost,
				query,
			),
			columns.description,
			&match_condition.description,
			query,
		)
	}
}

impl WriteWhereClause<Postgres, &MatchJob> for PgSchema
{
	/// # Panics
	///
	/// If any the following:
	///
	/// * `ident` is an empty string.
	///
	/// # See
	///
	/// * [`WriteWhereClause::write_where_clause`]
	fn write_where_clause(
		context: WriteContext,
		ident: impl Copy + Display,
		match_condition: &MatchJob,
		query: &mut QueryBuilder<Postgres>,
	) -> WriteContext
	{
		let columns = PgJobColumns::new().scoped(ident);

		PgSchema::write_where_clause(
			PgSchema::write_where_clause(
				PgSchema::write_where_clause(
					PgSchema::write_where_clause(
						PgSchema::write_where_clause(
							PgSchema::write_where_clause(
								PgSchema::write_where_clause(
									PgSchema::write_where_clause(
										PgSchema::write_where_clause(
											context,
											columns.id,
											&match_condition.id,
											query,
										),
										columns.date_close,
										&match_condition.date_close,
										query,
									),
									columns.date_open,
									&match_condition.date_open,
									query,
								),
								columns.increment,
								&match_condition.increment,
								query,
							),
							columns.invoice_date_issued,
							&match_condition.invoice.date_issued,
							query,
						),
						columns.invoice_date_paid,
						&match_condition.invoice.date_paid,
						query,
					),
					columns.invoice_hourly_rate,
					&match_condition.invoice.hourly_rate,
					query,
				),
				columns.notes,
				&match_condition.notes,
				query,
			),
			columns.objectives,
			&match_condition.objectives,
			query,
		)
	}
}

impl WriteWhereClause<Postgres, &MatchOrganization> for PgSchema
{
	/// # Panics
	///
	/// If any the following:
	///
	/// * `ident` is an empty string.
	///
	/// # See
	///
	/// * [`WriteWhereClause::write_where_clause`]
	fn write_where_clause(
		context: WriteContext,
		ident: impl Copy + Display,
		match_condition: &MatchOrganization,
		query: &mut QueryBuilder<Postgres>,
	) -> WriteContext
	{
		let columns = PgOrganizationColumns::new().scoped(ident);

		PgSchema::write_where_clause(
			PgSchema::write_where_clause(context, columns.id, &match_condition.id, query),
			columns.name,
			&match_condition.name,
			query,
		)
	}
}

impl WriteWhereClause<Postgres, &MatchTimesheet> for PgSchema
{
	/// # Panics
	///
	/// If any the following:
	///
	/// * `ident` is an empty string.
	///
	/// # See
	///
	/// * [`WriteWhereClause::write_where_clause`]
	fn write_where_clause(
		context: WriteContext,
		ident: impl Copy + Display,
		match_condition: &MatchTimesheet,
		query: &mut QueryBuilder<Postgres>,
	) -> WriteContext
	{
		let columns = PgTimesheetColumns::new().scoped(ident);

		PgSchema::write_where_clause(
			PgSchema::write_where_clause(
				PgSchema::write_where_clause(
					PgSchema::write_where_clause(context, columns.id, &match_condition.id, query),
					columns.time_begin,
					&match_condition.time_begin,
					query,
				),
				columns.time_end,
				&match_condition.time_end,
				query,
			),
			columns.work_notes,
			&match_condition.work_notes,
			query,
		)
	}
}
