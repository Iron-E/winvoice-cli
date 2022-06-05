use clinvoice_adapter::{schema::columns::JobColumns, Updatable};
use clinvoice_finance::{ExchangeRates, Exchangeable};
use clinvoice_schema::{
	chrono::{DateTime, Utc},
	Job,
};
use futures::TryFutureExt;
use sqlx::{Postgres, QueryBuilder, Result, Transaction};

use super::PgJob;
use crate::schema::{util, PgOrganization};

#[async_trait::async_trait]
impl Updatable for PgJob
{
	type Db = Postgres;
	type Entity = Job;

	async fn update<'e, 'i>(
		connection: &mut Transaction<Self::Db>,
		entities: impl 'async_trait + Clone + Iterator<Item = &'i Self::Entity> + Send,
	) -> Result<()>
	where
		'e: 'i,
		Self::Entity: 'e,
	{
		const COLUMNS: JobColumns<&'static str> = JobColumns::default();
		const TABLE_IDENT: &str = "J";
		const VALUES_IDENT: &str = "V";

		let mut peekable_entities = entities.clone().peekable();

		// There is nothing to do.
		if peekable_entities.peek().is_none()
		{
			return Ok(());
		}

		let exchange_rates_fut = ExchangeRates::new().map_err(util::finance_err_to_sqlx);

		let values_columns = COLUMNS.scoped(VALUES_IDENT);

		let mut query = QueryBuilder::new("UPDATE jobs AS ");
		query
			.separated(' ')
			.push(TABLE_IDENT)
			.push("SET")
			.push(COLUMNS.client_id)
			.push_unseparated('=')
			.push_unseparated(values_columns.client_id)
			.push_unseparated(',')
			.push_unseparated(COLUMNS.date_open)
			.push_unseparated('=')
			.push_unseparated(values_columns.date_open)
			.push_unseparated(',')
			.push_unseparated(COLUMNS.date_close)
			.push_unseparated('=')
			.push_unseparated(values_columns.date_close)
			.push_unseparated(',')
			.push_unseparated(COLUMNS.increment)
			.push_unseparated('=')
			.push_unseparated(values_columns.increment)
			.push_unseparated(',')
			.push_unseparated(COLUMNS.invoice_date_issued)
			.push_unseparated('=')
			.push_unseparated(values_columns.invoice_date_issued)
			.push_unseparated(',')
			.push_unseparated(COLUMNS.invoice_date_paid)
			.push_unseparated('=')
			.push_unseparated(values_columns.invoice_date_paid)
			.push_unseparated(',')
			.push_unseparated(COLUMNS.invoice_hourly_rate)
			.push_unseparated('=')
			.push_unseparated(values_columns.invoice_hourly_rate)
			.push_unseparated(',')
			.push_unseparated(COLUMNS.notes)
			.push_unseparated('=')
			.push_unseparated(values_columns.notes)
			.push_unseparated(',')
			.push_unseparated(COLUMNS.objectives)
			.push_unseparated('=')
			.push_unseparated(values_columns.objectives)
			.push("FROM (");

		let exchange_rates = exchange_rates_fut.await?;
		query.push_values(peekable_entities, |mut q, e| {
			q.push_bind(e.client.id)
				.push_bind(e.date_open)
				.push_bind(e.date_close)
				.push_bind(e.id)
				.push_bind(e.increment);

			if let Some(ref date) = e.invoice.date
			{
				q.push_bind(date.issued).push_bind(date.paid);
			}
			else
			{
				q.push_bind(None::<DateTime<Utc>>)
					.push_bind(None::<DateTime<Utc>>);
			}

			q.push_bind(
				e.invoice
					.hourly_rate
					.exchange(Default::default(), &exchange_rates)
					.amount
					.to_string(),
			)
			.push_bind(&e.notes)
			.push_bind(&e.objectives);
		});

		query
			.separated(' ')
			.push(") AS")
			.push(VALUES_IDENT)
			.push('(')
			.push_unseparated(COLUMNS.client_id)
			.push_unseparated(',')
			.push_unseparated(COLUMNS.date_open)
			.push_unseparated(',')
			.push_unseparated(COLUMNS.date_close)
			.push_unseparated(',')
			.push_unseparated(COLUMNS.id)
			.push_unseparated(',')
			.push_unseparated(COLUMNS.increment)
			.push_unseparated(',')
			.push_unseparated(COLUMNS.invoice_date_issued)
			.push_unseparated(',')
			.push_unseparated(COLUMNS.invoice_date_paid)
			.push_unseparated(',')
			.push_unseparated(COLUMNS.invoice_hourly_rate)
			.push_unseparated(',')
			.push_unseparated(COLUMNS.notes)
			.push_unseparated(',')
			.push_unseparated(COLUMNS.objectives)
			.push_unseparated(')')
			.push("WHERE")
			.push(COLUMNS.scoped(TABLE_IDENT).id)
			.push('=')
			.push(values_columns.id);

		query.push(';').build().execute(&mut *connection).await?;

		PgOrganization::update(connection, entities.map(|e| &e.client)).await?;

		Ok(())
	}
}

#[cfg(test)]
mod tests
{
	#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
	async fn update()
	{
		// TODO: write test
	}
}
