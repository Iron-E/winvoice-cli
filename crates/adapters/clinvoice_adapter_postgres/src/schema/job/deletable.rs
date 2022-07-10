use clinvoice_adapter::{schema::columns::JobColumns, Deletable};
use clinvoice_schema::{Id, Job};
use sqlx::{Executor, Postgres, Result};

use super::PgJob;
use crate::PgSchema;

#[async_trait::async_trait]
impl Deletable for PgJob
{
	type Db = Postgres;
	type Entity = Job;

	async fn delete<'e, 'i>(
		connection: impl 'async_trait + Executor<'_, Database = Self::Db>,
		entities: impl 'async_trait + Iterator<Item = &'i Self::Entity> + Send,
	) -> Result<()>
	where
		'e: 'i,
		Self::Entity: 'e,
	{
		fn mapper(j: &Job) -> Id
		{
			j.id
		}

		// TODO: use `for<'a> |e: &'a Job| e.id`
		PgSchema::delete::<_, _, JobColumns<char>>(connection, entities.map(mapper)).await
	}
}

#[cfg(test)]
mod tests
{
	use core::time::Duration;

	use clinvoice_adapter::{
		schema::{JobAdapter, LocationAdapter, OrganizationAdapter},
		Deletable,
	};
	use clinvoice_finance::{Currency, ExchangeRates, Exchangeable, Money};
	use clinvoice_match::{Match, MatchJob};
	use clinvoice_schema::{
		chrono::{TimeZone, Utc},
		Invoice,
		InvoiceDate,
	};
	use pretty_assertions::assert_eq;

	use crate::schema::{util, PgJob, PgLocation, PgOrganization};

	#[tokio::test]
	async fn delete()
	{
		let connection = util::connect().await;

		let earth = PgLocation::create(&connection, "Earth".into(), None)
			.await
			.unwrap();

		let organization =
			PgOrganization::create(&connection, earth.clone(), "Some Organization".into())
				.await
				.unwrap();

		let (job, job2, job3) = futures::try_join!(
			PgJob::create(
				&connection,
				organization.clone(),
				Some(Utc.ymd(1990, 08, 01).and_hms(09, 00, 00)),
				Utc.ymd(1990, 07, 12).and_hms(14, 10, 00),
				Duration::from_secs(300),
				Invoice {
					date: None,
					hourly_rate: Money::new(20_00, 2, Currency::Usd),
				},
				String::new(),
				"Do something".into()
			),
			PgJob::create(
				&connection,
				organization.clone(),
				Some(Utc.ymd(3000, 01, 16).and_hms(10, 00, 00)),
				Utc.ymd(3000, 01, 12).and_hms(09, 15, 42),
				Duration::from_secs(900),
				Invoice {
					date: Some(InvoiceDate {
						issued: Utc.ymd(3000, 01, 17).and_hms(12, 30, 00),
						paid: None,
					}),
					hourly_rate: Money::new(299_99, 2, Currency::Jpy),
				},
				String::new(),
				"Do something".into()
			),
			PgJob::create(
				&connection,
				organization.clone(),
				Some(Utc.ymd(2011, 03, 17).and_hms(13, 07, 07)),
				Utc.ymd(2011, 03, 17).and_hms(13, 07, 07),
				Duration::from_secs(900),
				Invoice {
					date: Some(InvoiceDate {
						issued: Utc.ymd(2011, 03, 18).and_hms(08, 00, 00),
						paid: Some(Utc.ymd(2011, 03, 19).and_hms(17, 00, 00)),
					}),
					hourly_rate: Money::new(20_00, 2, Currency::Eur),
				},
				String::new(),
				"Do something".into()
			),
		)
		.unwrap();

		assert!(
			PgOrganization::delete(&connection, [&organization].into_iter())
				.await
				.is_err()
		);
		PgJob::delete(&connection, [&job, &job2].into_iter())
			.await
			.unwrap();

		let exchange_rates = ExchangeRates::new().await.unwrap();
		assert_eq!(
			PgJob::retrieve(&connection, &MatchJob {
				id: Match::Or(vec![job.id.into(), job2.id.into(), job3.id.into()]),
				..Default::default()
			})
			.await
			.unwrap()
			.as_slice(),
			&[job3.exchange(Default::default(), &exchange_rates)],
		);
	}
}
