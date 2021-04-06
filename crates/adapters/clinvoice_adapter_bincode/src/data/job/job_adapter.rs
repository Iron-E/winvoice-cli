use
{
	std::{fs, io::BufReader},

	super::BincodeJob,
	crate::
	{
		data::{Error, Result},
		util,
	},

	clinvoice_adapter::
	{
		data::{Initializable, JobAdapter, query, Updatable},
		Store
	},
	clinvoice_data::
	{
		chrono::{DateTime, Utc},
		Invoice, Job, Money, Organization
	},
};

impl<'store> JobAdapter<'store> for BincodeJob<'_, 'store>
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
	fn create(
		client: Organization,
		date_open: DateTime<Utc>,
		hourly_rate: Money,
		objectives: &str,
		store: &'store Store,
	) -> Result<Job>
	{
		Self::init(&store)?;

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
			objectives: objectives.into(),
			notes: "".into(),
			timesheets: Vec::new(),
		};

		{
			BincodeJob {job: &job, store}.update()?;
		}

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
	fn retrieve(query: query::Job, store: &Store) -> Result<Vec<Job>>
	{
		Self::init(&store)?;

		let mut results = Vec::new();

		for node_path in util::read_files(BincodeJob::path(&store))?
		{
			let job: Job = bincode::deserialize_from(BufReader::new(
				fs::File::open(node_path)?
			))?;

			if query.matches(&job)
			{
				results.push(job);
			}
		}

		Ok(results)
	}
}

#[cfg(test)]
mod tests
{
	use
	{
		std::{borrow::Cow, fs, time::Instant},

		super::{BincodeJob, Job, JobAdapter, Money, Organization, query, Store, Utc, util},

		clinvoice_adapter::data::Match,
		clinvoice_data::{Decimal, Id},
	};

	#[test]
	fn create()
	{
		let organization = Organization
		{
			id: Id::new_v4(),
			location_id: Id::new_v4(),
			name: "Big Old Test Corporation".into(),
		};

		util::temp_store(|store|
		{
			let start = Instant::now();

			create_assertion(
				BincodeJob::create(
					organization.clone(),
					Utc::now(),
					Money::new(Decimal::new(200, 2), ""),
					"Test the job creation function",
					&store,
				).unwrap(),
				&store,
			);

			create_assertion(
				BincodeJob::create(
					organization.clone(),
					Utc::now(),
					Money::new(Decimal::new(200, 2), "USD"),
					"Test the job creation function",
					&store,
				).unwrap(),
				&store,
			);

			create_assertion(
				BincodeJob::create(
					organization.clone(),
					Utc::now(),
					Money::new(Decimal::new(20000, 0), "YEN"),
					"TEST THE JOB CREATION FUNCTION",
					&store,
				).unwrap(),
				&store,
			);

			create_assertion(
				BincodeJob::create(
					organization.clone(),
					Utc::now(),
					Money::new(Decimal::new(500, 2), "CDN"),
					"test the job creation function",
					&store,
				).unwrap(),
				&store,
			);

			create_assertion(
				BincodeJob::create(
					organization.clone(),
					Utc::now(),
					Money::new(Decimal::new(1000, 2), "EUR"),
					"TeSt ThE jOb CrEaTiOn FuNcTiOn",
					&store,
				).unwrap(),
				&store,
			);

			println!("\n>>>>> BincodeJob::create {}us <<<<<\n", Instant::now().duration_since(start).as_micros() / 5);
		});
	}

	fn create_assertion(job: Job, store: &Store)
	{
		let read_result = fs::read(BincodeJob {job: &job, store}.filepath()).unwrap();
		assert_eq!(job, bincode::deserialize(&read_result).unwrap());
	}

	#[test]
	fn retrieve()
	{
		let organization = Organization
		{
			id: Id::new_v4(),
			location_id: Id::new_v4(),
			name: "Big Old Test Corporation".into(),
		};

		util::temp_store(|store|
		{
			let creation = BincodeJob::create(
				organization.clone(),
				Utc::now(),
				Money::new(Decimal::new(200, 2), "USD"),
				"Test the job creation function",
				&store,
			).unwrap();

			let retrieval = BincodeJob::create(
				organization.clone(),
				Utc::now(),
				Money::new(Decimal::new(200, 2), "USD"),
				"Test the job retrieval function",
				&store,
			).unwrap();

			let assertion = BincodeJob::create(
				organization.clone(),
				Utc::now(),
				Money::new(Decimal::new(20000, 0), "YEN"),
				"Assert something",
				&store,
			).unwrap();

			let start = Instant::now();

			// retrieve everything
			let everything = BincodeJob::retrieve(
				query::Job
				{
					client: query::Organization
					{
						id: Match::EqualTo(Cow::Borrowed(&organization.id)),
						..Default::default()
					},
					..Default::default()
				},
				&store,
			).unwrap();

			// retrieve retrieval and assertion
			let not_creation = BincodeJob::retrieve(
				query::Job
				{
					date_open: Match::HasNone(vec![Cow::Borrowed(&creation.date_open)].into_iter().collect()),
					id: Match::HasAny(vec![Cow::Borrowed(&retrieval.id), Cow::Borrowed(&assertion.id)].into_iter().collect()),
					..Default::default()
				},
				&store,
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
		});
	}
}
