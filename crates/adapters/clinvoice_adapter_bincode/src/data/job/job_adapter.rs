use
{
	super::BincodeJob,
	crate::
	{
		data::{Error, Result},
		util,
	},

	clinvoice_adapter::
	{
		data::{Error as DataError, Initializable, JobAdapter, Updatable},
		Store
	},
	clinvoice_data::
	{
		chrono::{DateTime, Utc},
		Invoice, Job, finance::Money, Organization
	},
	clinvoice_query as query,
};

#[async_trait::async_trait]
impl JobAdapter for BincodeJob<'_, '_>
{
	type Error = Error;

	/// # Summary
	///
	/// Create a new [`Person`] on the active [`Store`](crate::Store).
	///
	/// # Paramters
	///
	/// See [`Job`].
	///
	/// # Returns
	///
	/// The newly created [`Person`].
	async fn create(
		client: Organization,
		date_open: DateTime<Utc>,
		hourly_rate: Money,
		objectives: String,
		store: &Store,
	) -> Result<Job>
	{
		let init_fut = Self::init(&store);

		let job = Job
		{
			client_id: client.id,
			date_close: None,
			date_open,
			id: util::unique_id(&Self::path(&store))?,
			invoice: Invoice
			{
				date: None,
				hourly_rate,
			},
			objectives,
			notes: "".into(),
			timesheets: Vec::new(),
		};

		init_fut.await?;
		BincodeJob {job: &job, store}.update().await?;

		Ok(job)
	}

	/// # Summary
	///
	/// Retrieve some [`Person`] from the active [`Store`](crate::Store).
	///
	/// # Parameters
	///
	/// See [`Job`].
	///
	/// # Returns
	///
	/// * An `Error`, if something goes wrong.
	/// * A list of matching [`Job`]s.
	async fn retrieve(query: &query::Job, store: &Store) -> Result<Vec<Job>>
	{
		Self::init(&store).await?;

		util::retrieve(Self::path(store),
			|j| query.matches(j).map_err(|e| DataError::from(e).into())
		).await
	}
}

#[cfg(test)]
mod tests
{
	use
	{
		std::
		{
			borrow::Cow::{Borrowed, Owned},
			time::Instant,
		},

		super::{BincodeJob, Job, JobAdapter, Money, Organization, query, Store, Utc, util},

		clinvoice_query::Match,
		clinvoice_data::{finance::Currency, Id},

		tokio::fs,
	};

	#[tokio::test]
	async fn create()
	{
		let organization = Organization
		{
			id: Id::new_v4(),
			location_id: Id::new_v4(),
			name: "Big Old Test Corporation".into(),
		};

		let store = util::temp_store();

		let start = Instant::now();

		let (test1, test2, test3, test4, test5) = futures::try_join!(
			BincodeJob::create(
				organization.clone(),
				Utc::now(),
				Money::new(2_00, 2, Currency::USD),
				"Test the job creation function".into(),
				&store,
			),

			BincodeJob::create(
				organization.clone(),
				Utc::now(),
				Money::new(2_00, 2, Currency::USD),
				"Test the job creation function".into(),
				&store,
			),

			BincodeJob::create(
				organization.clone(),
				Utc::now(),
				Money::new(20000, 0, Currency::JPY),
				"TEST THE JOB CREATION FUNCTION".into(),
				&store,
			),

			BincodeJob::create(
				organization.clone(),
				Utc::now(),
				Money::new(5_00, 2, Currency::CAD),
				"test the job creation function".into(),
				&store,
			),

			BincodeJob::create(
				organization.clone(),
				Utc::now(),
				Money::new(10_00, 2, Currency::EUR),
				"TeSt ThE jOb CrEaTiOn FuNcTiOn".into(),
				&store,
			),
		).unwrap();

		println!("\n>>>>> BincodeJob::create {}us <<<<<\n", Instant::now().duration_since(start).as_micros() / 5);

		futures::join!(
			create_assertion(test1, &store),
			create_assertion(test2, &store),
			create_assertion(test3, &store),
			create_assertion(test4, &store),
			create_assertion(test5, &store),
		);
	}

	async fn create_assertion(job: Job, store: &Store)
	{
		let read_result = fs::read(BincodeJob {job: &job, store}.filepath()).await.unwrap();
		assert_eq!(job, bincode::deserialize(&read_result).unwrap());
	}

	#[tokio::test]
	async fn retrieve()
	{
		let organization = Organization
		{
			id: Id::new_v4(),
			location_id: Id::new_v4(),
			name: "Big Old Test Corporation".into(),
		};

		let store = util::temp_store();

		let (creation, retrieval, assertion) =  futures::try_join!(
			BincodeJob::create(
				organization.clone(),
				Utc::now(),
				Money::new(2_00, 2, Currency::USD),
				"Test the job creation function".into(),
				&store,
			),

			BincodeJob::create(
				organization.clone(),
				Utc::now(),
				Money::new(2_00, 2, Currency::USD),
				"Test the job retrieval function".into(),
				&store,
			),

			BincodeJob::create(
				organization.clone(),
				Utc::now(),
				Money::new(20000, 0, Currency::JPY),
				"Assert something".into(),
				&store,
			),
		).unwrap();

		let start = Instant::now();

		let (everything, not_creation) = futures::try_join!(
			// retrieve everything
			BincodeJob::retrieve(
				&query::Job
				{
					client: query::Organization
					{
						id: Match::EqualTo(Borrowed(&organization.id)),
						..Default::default()
					},
					..Default::default()
				},
				&store,
			),

			// retrieve retrieval and assertion
			BincodeJob::retrieve(
				&query::Job
				{
					date_open: Match::Not(Match::HasAny(vec![
					  Owned(creation.date_open.naive_local()),
					].into_iter().collect()).into()),
					id: Match::HasAny(vec![Borrowed(&retrieval.id), Borrowed(&assertion.id)].into_iter().collect()),
					..Default::default()
				},
				&store,
			),
		).unwrap();

		println!("\n>>>>> BincodeJob::retrieve {}us <<<<<\n", Instant::now().duration_since(start).as_micros() / 2);

		// assert the results are as expected
		assert!(everything.contains(&assertion));
		assert!(everything.contains(&creation));
		assert!(everything.contains(&retrieval));

		// assert the results are as expected
		assert!(not_creation.contains(&assertion));
		assert!(!not_creation.contains(&creation));
		assert!(not_creation.contains(&retrieval));
	}
}
