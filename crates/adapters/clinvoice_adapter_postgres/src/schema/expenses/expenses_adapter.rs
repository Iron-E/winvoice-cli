use core::fmt::Write;
use std::collections::HashMap;

use clinvoice_adapter::{schema::ExpensesAdapter, WriteWhereClause};
use clinvoice_finance::{ExchangeRates, Money};
use clinvoice_match::{MatchExpense, MatchSet};
use clinvoice_schema::{Expense, Id};
use futures::{future, stream, StreamExt, TryFutureExt, TryStreamExt};
use sqlx::{Executor, PgPool, Postgres, Result, Row};

use super::{columns::PgExpenseColumns, PgExpenses};
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

		let exchange_rates_fut = ExchangeRates::new().map_err(util::finance_err_to_sqlx);

		const INSERT_VALUES_APPROX_LEN: u8 = 60;
		let mut expense_values =
			String::with_capacity((INSERT_VALUES_APPROX_LEN as usize) * expenses.len());

		// NOTE: `i * 4` is the number of values each iteration inserts
		(0..expenses.len()).map(|i| i * 4).for_each(|i| {
			write!(
				expense_values,
				"(${}, ${}, ${}, ${}),",
				i + 1,
				i + 2,
				i + 3,
				i + 4,
			)
			.unwrap()
		});
		expense_values.pop(); // get rid of the trailing `,` since SQL can't handle that :/

		let exchange_rates = exchange_rates_fut.await?;
		expenses
			.iter()
			.fold(
				sqlx::query(&format!(
					"INSERT INTO contact_information
						(timesheet_id, category, cost, description)
					VALUES {expense_values}
					RETURNING id;",
				)),
				|query, (category, cost, description)| {
					query
						.bind(timesheet_id)
						.bind(category)
						.bind(
							cost
								.exchange(Default::default(), &exchange_rates)
								.amount
								.to_string(),
						)
						.bind(description)
				},
			)
			.fetch(connection)
			.zip(stream::iter(expenses.iter().cloned()))
			.map(|(result, (category, cost, description))| {
				result.map(|row| Expense {
					category,
					cost,
					description,
					id: row.get::<Id, _>("id"),
					timesheet_id,
				})
			})
			.try_collect::<Vec<_>>()
			.await
	}

	async fn retrieve(
		connection: &PgPool,
		match_condition: MatchSet<MatchExpense>,
	) -> Result<HashMap<Id, Vec<Expense>>>
	{
		let mut query = String::from(
			"SELECT
				T.id as timesheet_id,
				X.category,
				X.cost,
				X.description,
				X.id
			FROM timesheets T
			LEFT JOIN expenses X ON (X.timesheet_id = T.id)",
		);
		PgSchema::write_where_clause(Default::default(), "X", &match_condition, &mut query);
		query.push(';');

		const COLUMNS: PgExpenseColumns<'static> = PgExpenseColumns {
			id: "id",
			timesheet_id: "timesheet_id",
			category: "category",
			cost: "cost",
			description: "description",
		};

		sqlx::query(&query)
			.fetch(connection)
			.try_fold(HashMap::new(), |mut map, row| {
				let entry = map.entry(row.get::<Id, _>(COLUMNS.timesheet_id)).or_insert_with(|| Vec::with_capacity(1));
				match COLUMNS.row_to_view(&row)
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
