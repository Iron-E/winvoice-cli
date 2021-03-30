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
		data::{Initializable, JobAdapter, Match, Updatable},
		Store
	},
	clinvoice_data::
	{
		chrono::{DateTime, Utc},
		Invoice, InvoiceDate, Job, Money, Organization, Id
	},
	std::{fs, io::BufReader},
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
	fn retrieve(
		client: Match<Id>,
		date_close: Match<Option<DateTime<Utc>>>,
		date_open: Match<DateTime<Utc>>,
		id: Match<Id>,
		invoice_date: Match<Option<InvoiceDate>>,
		invoice_hourly_rate: Match<Money>,
		notes: Match<String>,
		objectives: Match<String>,
		timesheet_employee: Match<Id>,
		timesheet_begin: Match<DateTime<Utc>>,
		timesheet_end: Match<Option<DateTime<Utc>>>,
		store: &Store,
	) -> Result<Vec<Job>>
	{
		Self::init(&store)?;

		let mut results = Vec::new();

		for node_path in util::read_files(BincodeJob::path(&store))?
		{
			let job: Job = bincode::deserialize_from(BufReader::new(
				fs::File::open(node_path)?
			))?;

			if client.matches(&job.client_id) &&
				date_close.matches(&job.date_close) &&
				date_open.matches(&job.date_open) &&
				id.matches(&job.id) &&
				invoice_date.matches(&job.invoice.date) &&
				invoice_hourly_rate.matches(&job.invoice.hourly_rate) &&
				notes.matches(&job.notes) &&
				objectives.matches(&job.objectives) &&
				timesheet_employee.set_matches(&job.timesheets.iter().map(|t| &t.employee_id).collect()) &&
				timesheet_begin.set_matches(&job.timesheets.iter().map(|t| &t.time_begin).collect()) &&
				timesheet_end.set_matches(&job.timesheets.iter().map(|t| &t.time_end).collect())
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
		super::{BincodeJob, Id, Job, JobAdapter, Match, Money, Organization, Store, Utc, util},
		clinvoice_data::Decimal,
		std::{borrow::Cow, fs, time::Instant},
	};

	#[test]
	fn test_create()
	{
		let organization = Organization
		{
			id: Id::new_v4(),
			location_id: Id::new_v4(),
			name: "Big Old Test Corporation".into(),
		};

		util::test_temp_store(|store|
		{
			let start = Instant::now();

			test_create_assertion(
				BincodeJob::create(
					organization.clone(),
					Utc::now(),
					Money::new(Decimal::new(200, 2), ""),
					"Test the job creation function",
					&store,
				).unwrap(),
				&store,
			);

			test_create_assertion(
				BincodeJob::create(
					organization.clone(),
					Utc::now(),
					Money::new(Decimal::new(200, 2), "USD"),
					"Test the job creation function",
					&store,
				).unwrap(),
				&store,
			);

			test_create_assertion(
				BincodeJob::create(
					organization.clone(),
					Utc::now(),
					Money::new(Decimal::new(20000, 0), "YEN"),
					"TEST THE JOB CREATION FUNCTION",
					&store,
				).unwrap(),
				&store,
			);

			test_create_assertion(
				BincodeJob::create(
					organization.clone(),
					Utc::now(),
					Money::new(Decimal::new(500, 2), "CDN"),
					"test the job creation function",
					&store,
				).unwrap(),
				&store,
			);

			test_create_assertion(
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

	fn test_create_assertion(job: Job, store: &Store)
	{
		let read_result = fs::read(BincodeJob {job: &job, store}.filepath()).unwrap();
		assert_eq!(job, bincode::deserialize(&read_result).unwrap());
	}

	#[test]
	fn test_retrieve()
	{
		let organization = Organization
		{
			id: Id::new_v4(),
			location_id: Id::new_v4(),
			name: "Big Old Test Corporation".into(),
		};

		util::test_temp_store(|store|
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
				Match::EqualTo(Cow::Borrowed(&organization.id)), // client
				Match::Any, // date close
				Match::Any, // date open
				Match::Any, // id
				Match::Any, // invoice date
				Match::Any, // invoice hourly rate
				Match::Any, // notes
				Match::Any, // objectives
				Match::Any, // timesheet employee
				Match::Any, // timesheet time begin
				Match::Any, // timesheet time end
				&store,
			).unwrap();

			// retrieve retrieval and assertion
			let not_creation = BincodeJob::retrieve(
				Match::Any, // client
				Match::Any, // date close
				Match::HasNone(vec![Cow::Borrowed(&creation.date_open)].into_iter().collect()), // date open
				Match::HasAny(vec![Cow::Borrowed(&retrieval.id), Cow::Borrowed(&assertion.id)].into_iter().collect()), // id
				Match::Any, // invoice date
				Match::Any, // invoice hourly rate
				Match::Any, // notes
				Match::Any, // objectives
				Match::Any, // timesheet employee
				Match::Any, // timesheet time begin
				Match::Any, // timesheet time end
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
