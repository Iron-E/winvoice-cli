use clinvoice_adapter::{
	fmt::{sql, QueryBuilderExt, TableToSql},
	schema::{columns::ExpenseColumns, ExpensesAdapter},
	WriteWhereClause,
};
use clinvoice_finance::{ExchangeRates, Exchangeable, Money};
use clinvoice_match::MatchExpense;
use clinvoice_schema::{Expense, Id};
use futures::{future, stream, StreamExt, TryFutureExt, TryStreamExt};
use sqlx::{Executor, PgPool, Postgres, QueryBuilder, Result, Row};

use super::PgExpenses;
use crate::{schema::util, PgSchema};

const COLUMNS: ExpenseColumns<&'static str> = ExpenseColumns::default();

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

		QueryBuilder::new(
			"INSERT INTO expenses
				(timesheet_id, category, cost, description) ",
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
		.push(sql::RETURNING)
		.push(COLUMNS.id)
		.prepare()
		.fetch(connection)
		.zip(stream::iter(expenses.iter()))
		.map(|(result, (category, cost, description))| {
			result.map(|row| Expense {
				category: category.clone(),
				cost: *cost,
				description: description.clone(),
				id: row.get(COLUMNS.id),
				timesheet_id,
			})
		})
		.try_collect::<Vec<_>>()
		.await
	}

	async fn retrieve(connection: &PgPool, match_condition: &MatchExpense) -> Result<Vec<Expense>>
	{
		const COLUMNS: ExpenseColumns<&str> = ExpenseColumns::default();

		let columns = COLUMNS.default_scope();
		let exchange_rates_fut = ExchangeRates::new().map_err(util::finance_err_to_sqlx);
		let mut query = QueryBuilder::new(sql::SELECT);

		query
			.push_columns(&columns)
			.push_default_from::<ExpenseColumns<char>>();

		let exchange_rates = exchange_rates_fut.await?;
		PgSchema::write_where_clause(
			Default::default(),
			ExpenseColumns::<char>::DEFAULT_ALIAS,
			&match_condition.exchange_ref(Default::default(), &exchange_rates),
			&mut query,
		);

		query
			.prepare()
			.fetch(connection)
			.and_then(|row| future::ready(PgExpenses::row_to_view(COLUMNS, &row)))
			.try_collect()
			.await
	}
}
