use clinvoice_adapter::{schema::columns::JobColumns, Updatable};
use clinvoice_finance::{ExchangeRates, Exchangeable};
use clinvoice_schema::{
	chrono::{DateTime, Utc},
	Job,
};
use futures::TryFutureExt;
use sqlx::{Postgres, Result, Transaction};

use super::PgJob;
use crate::{
	schema::{util, PgOrganization},
	PgSchema,
};

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
		let mut peekable_entities = entities.clone().peekable();

		// There is nothing to do.
		if peekable_entities.peek().is_none()
		{
			return Ok(());
		}

		const COLUMNS: JobColumns<&'static str> = JobColumns::default();
		let exchange_rates = ExchangeRates::new()
			.map_err(util::finance_err_to_sqlx)
			.await?;
		PgSchema::update(&mut *connection, COLUMNS, "jobs", "J", "V", |query| {
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
		})
		.await?;

		PgOrganization::update(connection, entities.map(|e| &e.client)).await?;

		Ok(())
	}
}

#[cfg(test)]
mod tests
{
	#[tokio::test]
	async fn update()
	{
		// TODO: write test
	}
}
