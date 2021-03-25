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
		data::{Initializable, JobAdapter, MatchWhen, Updatable},
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
		client: MatchWhen<Id>,
		date_close: MatchWhen<Option<DateTime<Utc>>>,
		date_open: MatchWhen<DateTime<Utc>>,
		id: MatchWhen<Id>,
		invoice_date: MatchWhen<Option<InvoiceDate>>,
		invoice_hourly_rate: MatchWhen<Money>,
		notes: MatchWhen<String>,
		objectives: MatchWhen<String>,
		timesheet_employee: MatchWhen<Id>,
		timesheet_begin: MatchWhen<DateTime<Utc>>,
		timesheet_end: MatchWhen<Option<DateTime<Utc>>>,
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

			if client.is_match(&job.client_id) &&
				date_close.is_match(&job.date_close) &&
				date_open.is_match(&job.date_open) &&
				id.is_match(&job.id) &&
				invoice_date.is_match(&job.invoice.date) &&
				invoice_hourly_rate.is_match(&job.invoice.hourly_rate) &&
				notes.is_match(&job.notes) &&
				objectives.is_match(&job.objectives) &&
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
		super::{BincodeJob, Id, Job, JobAdapter, MatchWhen, Money, Organization, Store, Utc, util},
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
				MatchWhen::EqualTo(Cow::Borrowed(&organization.id)), // client
				MatchWhen::Any, // date close
				MatchWhen::Any, // date open
				MatchWhen::Any, // id
				MatchWhen::Any, // invoice date
				MatchWhen::Any, // invoice hourly rate
				MatchWhen::Any, // notes
				MatchWhen::Any, // objectives
				MatchWhen::Any, // timesheet employee
				MatchWhen::Any, // timesheet time begin
				MatchWhen::Any, // timesheet time end
				&store,
			).unwrap();

			// retrieve retrieval and assertion
			let not_creation = BincodeJob::retrieve(
				MatchWhen::Any, // client
				MatchWhen::Any, // date close
				MatchWhen::HasNone(vec![Cow::Borrowed(&creation.date_open)].into_iter().collect()), // date open
				MatchWhen::HasAny(vec![Cow::Borrowed(&retrieval.id), Cow::Borrowed(&assertion.id)].into_iter().collect()), // id
				MatchWhen::Any, // invoice date
				MatchWhen::Any, // invoice hourly rate
				MatchWhen::Any, // notes
				MatchWhen::Any, // objectives
				MatchWhen::Any, // timesheet employee
				MatchWhen::Any, // timesheet time begin
				MatchWhen::Any, // timesheet time end
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
