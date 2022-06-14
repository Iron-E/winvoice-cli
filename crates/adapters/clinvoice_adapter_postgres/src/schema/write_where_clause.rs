use core::{fmt::Display, ops::Deref};

use async_recursion::async_recursion;
use clinvoice_adapter::{
	fmt::Nullable,
	schema::columns::{
		ContactColumns,
		EmployeeColumns,
		ExpenseColumns,
		JobColumns,
		OrganizationColumns,
		TimesheetColumns,
	},
	WriteContext,
	WriteWhereClause,
};
use clinvoice_finance::Decimal;
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
	MatchRow,
	MatchTimesheet,
};
use clinvoice_schema::Id;
use sqlx::{Database, PgPool, Postgres, QueryBuilder, Result};

use super::{PgLocation, PgSchema};
use crate::fmt::{PgInterval, PgTimestampTz};

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
		.push("IS null");
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
pub(super) async fn write_match_contact_set<A>(
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
				write_match_contact_set(connection, WriteContext::InWhereCondition, ident, c, query)
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
				write_match_contact_set(connection, WriteContext::InWhereCondition, ident, c, query)
					.await?;
			}

			write_context_scope_end(query);
		},

		MatchSet::Contains(match_contact) =>
		{
			const COLUMNS: ContactColumns<&'static str> = ContactColumns::default();

			let subquery_ident = format!("{ident}_2");
			let subquery_ident_columns = COLUMNS.scoped(&subquery_ident);

			query
				.separated(' ')
				.push(context)
				.push("EXISTS (SELECT FROM contact_information")
				.push(&subquery_ident)
				.push("WHERE")
				.push(subquery_ident_columns.organization_id)
				.push_unseparated('=')
				.push_unseparated(COLUMNS.scoped(ident).organization_id);

			let ctx = PgSchema::write_where_clause(
				PgSchema::write_where_clause(
					PgSchema::write_where_clause(
						WriteContext::AcceptingAnotherWhereCondition,
						subquery_ident_columns.export,
						&match_contact.export,
						query,
					),
					subquery_ident_columns.label,
					&match_contact.label,
					query,
				),
				subquery_ident_columns.organization_id,
				&match_contact.organization_id,
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
						subquery_ident_columns.address_id,
						&location_id_query,
						query,
					);
				},

				MatchContactKind::SomeEmail(ref email_address) =>
				{
					PgSchema::write_where_clause(
						ctx,
						subquery_ident_columns.email,
						email_address,
						query,
					);
				},

				MatchContactKind::SomePhone(ref phone_number) =>
				{
					PgSchema::write_where_clause(ctx, subquery_ident_columns.phone, phone_number, query);
				},
			};

			query.push(')');
		},

		MatchSet::Not(condition) => match condition.deref()
		{
			m if m.deref() == &Default::default() => write_is_null(query, context, ident),
			m =>
			{
				write_context_scope_start::<_, true>(query, context);

				write_match_contact_set(connection, WriteContext::InWhereCondition, ident, m, query)
					.await?;

				write_context_scope_end(query);
			},
		},
	};

	Ok(WriteContext::AcceptingAnotherWhereCondition)
}

/// # Summary
///
/// A blanket implementation of [`WriteWhereClause`] for [`Match`].
///
/// Requires access to something else that has already implemented [`WriteWhereClause`] for
/// [`PgSchema`], so that methods like [`write_boolean_group`] can be abstracted away from _this_
/// implementation.
///
/// # Warnings
///
/// Does not guard against SQL injection.
fn write_match<Db, T>(
	context: WriteContext,
	ident: impl Copy + Display,
	match_condition: &Match<T>,
	query: &mut QueryBuilder<Db>,
) -> WriteContext
where
	Db: Database,
	T: Display + PartialEq,
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
		Match::EqualTo(value) => write_comparison(query, context, ident, "=", value),
		Match::GreaterThan(value) => write_comparison(query, context, ident, ">", value),
		Match::InRange(low, high) =>
		{
			write_comparison(query, context, ident, "BETWEEN", low);
			write_comparison(query, WriteContext::InWhereCondition, "", "AND", high);
		},
		Match::LessThan(value) => write_comparison(query, context, ident, "<", value),
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

/// # Summary
///
/// A blanket implementation of [`WriteWhereClause`] for a [`MatchRow`] of another match
/// structure (e.g. [`MatchRow<MatchJob>`]).
fn write_match_row<Db, M>(
	context: WriteContext,
	ident: impl Copy + Display,
	match_condition: &MatchRow<M>,
	query: &mut QueryBuilder<Db>,
) -> WriteContext
where
	Db: Database,
	M: Default + PartialEq,
	MatchRow<M>: PartialEq,
	for<'a> PgSchema: WriteWhereClause<Db, &'a M>,
	for<'a> PgSchema: WriteWhereClause<Db, &'a MatchRow<M>>,
{
	match match_condition
	{
		MatchRow::And(conditions) | MatchRow::Or(conditions) =>
		{
			write_context_scope_start::<_, false>(query, context);

			if let Some(c) = conditions.first()
			{
				PgSchema::write_where_clause(WriteContext::InWhereCondition, ident, c, query);
			}

			let separator = match match_condition
			{
				MatchRow::And(_) => " AND",
				_ => " OR",
			};

			conditions.iter().skip(1).for_each(|c| {
				query.push(separator);
				PgSchema::write_where_clause(WriteContext::InWhereCondition, ident, c, query);
			});

			write_context_scope_end(query);
		},

		MatchRow::EqualTo(condition) =>
		{
			if condition.ne(&Default::default())
			{
				write_context_scope_start::<_, false>(query, context);
				PgSchema::write_where_clause(context, WriteContext::InWhereCondition, condition, query);
				write_context_scope_end(query);
			}
		},

		MatchRow::Not(condition) =>
		{
			let m = condition.deref();
			if m.ne(&Default::default())
			{
				write_context_scope_start::<_, true>(query, context);
				PgSchema::write_where_clause(context, ident, m, query);
				write_context_scope_end(query);
			}
		},
	};

	WriteContext::AcceptingAnotherWhereCondition
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

impl WriteWhereClause<Postgres, &Match<bool>> for PgSchema
{
	fn write_where_clause(
		context: WriteContext,
		ident: impl Copy + Display,
		match_condition: &Match<bool>,
		query: &mut QueryBuilder<Postgres>,
	) -> WriteContext
	{
		write_match(context, ident, match_condition, query)
	}
}

impl WriteWhereClause<Postgres, &Match<Decimal>> for PgSchema
{
	fn write_where_clause(
		context: WriteContext,
		ident: impl Copy + Display,
		match_condition: &Match<Decimal>,
		query: &mut QueryBuilder<Postgres>,
	) -> WriteContext
	{
		write_match(context, ident, match_condition, query)
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
		write_match(context, ident, match_condition, query)
	}
}

impl WriteWhereClause<Postgres, &Match<PgInterval>> for PgSchema
{
	fn write_where_clause(
		context: WriteContext,
		ident: impl Copy + Display,
		match_condition: &Match<PgInterval>,
		query: &mut QueryBuilder<Postgres>,
	) -> WriteContext
	{
		write_match(context, ident, match_condition, query)
	}
}

impl<T> WriteWhereClause<Postgres, &Match<Nullable<T>>> for PgSchema
where
	T: Display + PartialEq,
{
	fn write_where_clause(
		context: WriteContext,
		ident: impl Copy + Display,
		match_condition: &Match<Nullable<T>>,
		query: &mut QueryBuilder<Postgres>,
	) -> WriteContext
	{
		write_match(context, ident, match_condition, query)
	}
}

impl WriteWhereClause<Postgres, &Match<PgTimestampTz>> for PgSchema
{
	fn write_where_clause(
		context: WriteContext,
		ident: impl Copy + Display,
		match_condition: &Match<PgTimestampTz>,
		query: &mut QueryBuilder<Postgres>,
	) -> WriteContext
	{
		write_match(context, ident, match_condition, query)
	}
}

impl WriteWhereClause<Postgres, &MatchRow<MatchEmployee>> for PgSchema
{
	fn write_where_clause(
		context: WriteContext,
		ident: impl Copy + Display,
		match_condition: &MatchRow<MatchEmployee>,
		query: &mut QueryBuilder<Postgres>,
	) -> WriteContext
	{
		write_match_row(context, ident, match_condition, query)
	}
}

impl WriteWhereClause<Postgres, &MatchRow<MatchJob>> for PgSchema
{
	fn write_where_clause(
		context: WriteContext,
		ident: impl Copy + Display,
		match_condition: &MatchRow<MatchJob>,
		query: &mut QueryBuilder<Postgres>,
	) -> WriteContext
	{
		write_match_row(context, ident, match_condition, query)
	}
}

impl WriteWhereClause<Postgres, &MatchRow<MatchOrganization>> for PgSchema
{
	fn write_where_clause(
		context: WriteContext,
		ident: impl Copy + Display,
		match_condition: &MatchRow<MatchOrganization>,
		query: &mut QueryBuilder<Postgres>,
	) -> WriteContext
	{
		write_match_row(context, ident, match_condition, query)
	}
}

impl WriteWhereClause<Postgres, &MatchRow<MatchTimesheet>> for PgSchema
{
	fn write_where_clause(
		context: WriteContext,
		ident: impl Copy + Display,
		match_condition: &MatchRow<MatchTimesheet>,
		query: &mut QueryBuilder<Postgres>,
	) -> WriteContext
	{
		write_match_row(context, ident, match_condition, query)
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

				conditions.iter().for_each(|c| {
					query.push(separator);
					PgSchema::write_where_clause(WriteContext::InWhereCondition, ident, c, query);
				});

				write_context_scope_end(query);
			},

			MatchSet::Contains(match_expense) =>
			{
				const COLUMNS: ExpenseColumns<&'static str> = ExpenseColumns::default();

				let subquery_ident = format!("{ident}_2");
				let subquery_ident_columns = COLUMNS.scoped(&subquery_ident);

				query
					.separated(' ')
					.push(context)
					.push("EXISTS (SELECT FROM expenses")
					.push(&subquery_ident)
					.push("WHERE")
					.push(subquery_ident_columns.timesheet_id)
					.push_unseparated('=')
					.push_unseparated(COLUMNS.scoped(ident).timesheet_id);

				PgSchema::write_where_clause(
					WriteContext::AcceptingAnotherWhereCondition,
					&subquery_ident,
					match_expense,
					query,
				);

				query.push(')');
			},

			MatchSet::Not(condition) => match condition.deref()
			{
				m if m == &Default::default() => write_is_null(query, context, ident),
				m => write_negated(query, context, ident, m),
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
					.push_unseparated('=')
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
					.push_unseparated('~')
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
		let columns = EmployeeColumns::default().scoped(ident);

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
		let columns = ExpenseColumns::default().scoped(ident);

		PgSchema::write_where_clause(
			PgSchema::write_where_clause(
				PgSchema::write_where_clause(
					PgSchema::write_where_clause(
						PgSchema::write_where_clause(context, columns.id, &match_condition.id, query),
						columns.category,
						&match_condition.category,
						query,
					),
					// NOTE: `cost` is stored as text on the DB
					columns.typecast("numeric").cost,
					&match_condition.cost.map_ref(|c| c.amount),
					query,
				),
				columns.description,
				&match_condition.description,
				query,
			),
			columns.timesheet_id,
			&match_condition.timesheet_id,
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
		let columns = JobColumns::default().scoped(ident);

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
										&match_condition
											.date_close
											.map_ref(|d| Nullable(d.map(PgTimestampTz))),
										query,
									),
									columns.date_open,
									&match_condition.date_open.map_ref(|d| PgTimestampTz(*d)),
									query,
								),
								columns.increment,
								&match_condition
									.increment
									.map_ref(|i| PgInterval(i.into_inner())),
								query,
							),
							columns.invoice_date_issued,
							&match_condition
								.invoice
								.date_issued
								.map_ref(|d| Nullable(d.map(PgTimestampTz))),
							query,
						),
						columns.invoice_date_paid,
						&match_condition
							.invoice
							.date_paid
							.map_ref(|d| Nullable(d.map(PgTimestampTz))),
						query,
					),
					// NOTE: `hourly_rate` is stored as text on the DB
					columns.typecast("numeric").invoice_hourly_rate,
					&match_condition.invoice.hourly_rate.map_ref(|r| r.amount),
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
		let columns = OrganizationColumns::default().scoped(ident);

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
		let columns = TimesheetColumns::default().scoped(ident);

		PgSchema::write_where_clause(
			PgSchema::write_where_clause(
				PgSchema::write_where_clause(
					PgSchema::write_where_clause(context, columns.id, &match_condition.id, query),
					columns.time_begin,
					&match_condition.time_begin.map_ref(|d| PgTimestampTz(*d)),
					query,
				),
				columns.time_end,
				&match_condition
					.time_end
					.map_ref(|d| Nullable(d.map(PgTimestampTz))),
				query,
			),
			columns.work_notes,
			&match_condition.work_notes,
			query,
		)
	}
}
