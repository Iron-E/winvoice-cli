use std::collections::HashMap;

use clinvoice_adapter::{
	schema::{columns::ExpenseColumns, ExpensesAdapter},
	WriteWhereClause,
};
use clinvoice_finance::{ExchangeRates, Money, Exchangeable};
use clinvoice_match::{MatchExpense, MatchSet};
use clinvoice_schema::{Expense, Id};
use futures::{future, stream, StreamExt, TryFutureExt, TryStreamExt};
use sqlx::{Executor, PgPool, Postgres, QueryBuilder, Result, Row};

use super::PgExpenses;
use crate::{schema::util, PgSchema};

#[async_trait::async_trait]
impl ExpensesAdapter for PgExpenses
{
	async fn create(
		connection: impl 'async_trait + Executor<'_, Database = Postgres> + Send,
		expenses: Vec<(String, Money, String)>,
		timesheet_id: Id,
	) -> Result<Vec<Expense>>
	{
		if expenses.is_empty()
		{
			return Ok(Vec::new());
		}

		let exchange_rates = ExchangeRates::new()
			.map_err(util::finance_err_to_sqlx)
			.await?;

		const COLUMNS: ExpenseColumns<&'static str> = ExpenseColumns::default();

		QueryBuilder::new(
			"INSERT INTO contact_information
				(timesheet_id, category, cost, description)",
		)
		.push_values(expenses.iter(), |mut q, (category, cost, description)| {
			q.push_bind(timesheet_id)
				.push_bind(category)
				.push_bind(
					cost
						.exchange(Default::default(), &exchange_rates)
						.amount
						.to_string(),
				)
				.push_bind(description);
		})
		.push(';')
		.build()
		.fetch(connection)
		.zip(stream::iter(expenses.iter().cloned()))
		.map(|(result, (category, cost, description))| {
			result.map(|row| Expense {
				category,
				cost,
				description,
				id: row.get(COLUMNS.id),
				timesheet_id,
			})
		})
		.try_collect::<Vec<_>>()
		.await
	}

	async fn retrieve(
		connection: &PgPool,
		match_condition: &MatchSet<MatchExpense>,
	) -> Result<HashMap<Id, Vec<Expense>>>
	{
		let exchange_rates_fut = ExchangeRates::new().map_err(util::finance_err_to_sqlx);

		const COLUMNS: ExpenseColumns<&'static str> = ExpenseColumns::default();

		let mut query = QueryBuilder::new(
			"SELECT
				T.id as timesheet_id,
				X.category,
				X.cost,
				X.description,
				X.id
			FROM timesheets T
			LEFT JOIN expenses X ON (X.timesheet_id = T.id)",
		);

		let exchange_rates = exchange_rates_fut.await?;
		PgSchema::write_where_clause(
			Default::default(),
			"X",
			&match_condition.exchange(Default::default(), &exchange_rates),
			&mut query,
		);

		query
			.push(';')
			.build()
			.fetch(connection)
			.try_fold(HashMap::new(), |mut map, row| {
				let entry = map
					.entry(row.get::<Id, _>(COLUMNS.timesheet_id))
					.or_insert_with(|| Vec::with_capacity(1));
				match PgExpenses::row_to_view(COLUMNS, &row)
				{
					Ok(Some(expense)) => entry.push(expense),
					Err(e) => return future::err(e),
					_ => (),
				};

				future::ok(map)
			})
			.await
	}
}
