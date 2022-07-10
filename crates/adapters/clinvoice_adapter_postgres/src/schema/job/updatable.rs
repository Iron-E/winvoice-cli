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
	fmt::DateTimeExt,
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

		let exchange_rates = ExchangeRates::new()
			.map_err(util::finance_err_to_sqlx)
			.await?;
		PgSchema::update(connection, JobColumns::default(), |query| {
			query.push_values(peekable_entities, |mut q, e| {
				q.push_bind(e.client.id)
					.push_bind(e.date_open.pg_sanitize())
					.push_bind(e.date_close.pg_sanitize())
					.push_bind(e.id)
					.push_bind(e.increment);

				if let Some(ref date) = e.invoice.date.pg_sanitize()
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

		PgOrganization::update(connection, entities.map(|e| &e.client)).await
	}
}

#[cfg(test)]
mod tests
{
	use core::time::Duration;

	use clinvoice_adapter::{
		schema::{JobAdapter, LocationAdapter, OrganizationAdapter},
		Updatable,
	};
	use clinvoice_finance::Money;
	use clinvoice_match::MatchJob;
	use clinvoice_schema::{chrono::Utc, Invoice, InvoiceDate};
	use futures::TryFutureExt;
	use pretty_assertions::assert_eq;

	use crate::{
		fmt::DateTimeExt,
		schema::{util, PgJob, PgLocation, PgOrganization},
	};

	#[tokio::test]
	async fn update()
	{
		let connection = util::connect().await;

		let (earth, mars) = futures::try_join!(
			PgLocation::create(&connection, "Earth".into(), None),
			PgLocation::create(&connection, "Mars".into(), None),
		)
		.unwrap();

		let mut job = PgOrganization::create(&connection, earth, "Some Organization".into())
			.and_then(|organization| {
				PgJob::create(
					&connection,
					organization,
					None,
					Utc::now(),
					Duration::from_secs(900),
					Default::default(),
					Default::default(),
					Default::default(),
				)
			})
			.await
			.unwrap();

		job.client.location = mars;
		job.client.name = format!("Not {}", job.client.name);
		job.date_close = Some(Utc::now());
		job.increment = Duration::from_secs(300);
		job.invoice = Invoice {
			date: Some(InvoiceDate {
				issued: Utc::now(),
				paid: Some(Utc::now()),
			}),
			hourly_rate: Money::new(200_00, 2, Default::default()),
		};
		job.notes = format!("Finished {}", job.notes);
		job.objectives = format!("Test {}", job.notes);

		{
			let mut transaction = connection.begin().await.unwrap();
			PgJob::update(&mut transaction, [&job].into_iter())
				.await
				.unwrap();
			transaction.commit().await.unwrap();
		}

		let db_job = PgJob::retrieve(&connection, &MatchJob {
			id: job.id.into(),
			..Default::default()
		})
		.await
		.unwrap()
		.pop()
		.unwrap();

		assert_eq!(job.client, db_job.client);
		assert_eq!(job.date_close.pg_sanitize(), db_job.date_close);
		assert_eq!(job.date_open.pg_sanitize(), db_job.date_open);
		assert_eq!(job.id, db_job.id);
		assert_eq!(job.increment, db_job.increment);
		assert_eq!(job.invoice.pg_sanitize(), db_job.invoice);
		assert_eq!(job.notes, db_job.notes);
		assert_eq!(job.objectives, db_job.objectives);
	}
}
