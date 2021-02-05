use
{
	super::BincodeJob,
	crate::util,
	clinvoice_adapter::
	{
		data::{MatchWhen, JobAdapter, Updatable},
		Store
	},
	clinvoice_data::
	{
		chrono::{DateTime, Utc},
		Invoice, Job, Money, Organization, Id
	},
	std::
	{
		collections::{BTreeSet, HashSet},
		error::Error, fs, io::BufReader
	},
};

impl<'pass, 'path, 'user> JobAdapter<'pass, 'path, 'user> for BincodeJob<'pass, 'path, 'user>
{
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
	fn create<'objectives>(
		client: Organization,
		date_open: DateTime<Utc>,
		hourly_rate: Money,
		objectives: &'objectives str,
		store: Store<'pass, 'path, 'user>,
	) -> Result<Self, Box<dyn Error>>
	{
		Self::init(&store)?;

		let bincode_job = Self
		{
			job: Job
			{
				client_id: client.id,
				date_close: None,
				date_open,
				id: util::unique_id(&Self::path(&store))?,
				invoice: Invoice
				{
					date_issued: None,
					date_paid: None,
					hourly_rate,
				},
				objectives: objectives.into(),
				notes: "".into(),
				timesheets: BTreeSet::new(),
			},
			store,
		};

		bincode_job.update()?;

		return Ok(bincode_job);
	}

	/// # Summary
	///
	/// Initialize the database for a given [`Store`].
	fn init(store: &Store<'pass, 'path, 'user>) -> Result<(), Box<dyn Error>>
	{
		util::create_store_dir(&Self::path(store))?;
		return Ok(());
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
		invoice_date_issued: MatchWhen<Option<DateTime<Utc>>>,
		invoice_date_paid: MatchWhen<Option<DateTime<Utc>>>,
		invoice_hourly_rate: MatchWhen<Money>,
		notes: MatchWhen<String>,
		objectives: MatchWhen<String>,
		timesheet_employee: MatchWhen<Id>,
		timesheet_begin: MatchWhen<DateTime<Utc>>,
		timesheet_end: MatchWhen<Option<DateTime<Utc>>>,
		store: Store<'pass, 'path, 'user>,
	) -> Result<HashSet<Self>, Box<dyn Error>>
	{
		let mut results = HashSet::new();

		for node_path in fs::read_dir(BincodeJob::path(&store))?.filter_map(
			|node| match node {Ok(n) => Some(n.path()), Err(_) => None}
		)
		{
			let job: Job = bincode::deserialize_from(
				BufReader::new(fs::File::open(node_path)?
			))?;

			if client.is_match(&job.client_id) &&
				date_close.is_match(&job.date_close) &&
				date_open.is_match(&job.date_open) &&
				id.is_match(&job.id) &&
				invoice_date_issued.is_match(&job.invoice.date_issued) &&
				invoice_date_paid.is_match(&job.invoice.date_paid) &&
				invoice_hourly_rate.is_match(&job.invoice.hourly_rate) &&
				notes.is_match(&job.notes) &&
				objectives.is_match(&job.objectives) &&
				timesheet_employee.set_matches(&job.timesheets.iter().map(|t| t.employee_id).collect()) &&
				timesheet_begin.set_matches(&job.timesheets.iter().map(|t| t.time_begin).collect()) &&
				timesheet_end.set_matches(&job.timesheets.iter().map(|t| t.time_end).collect())
			{
				results.insert(BincodeJob {job, store});
			}
		}

		return Ok(results);
	}
}

#[cfg(test)]
mod tests
{
	use
	{
		super::{BincodeJob, Id, HashSet, JobAdapter, MatchWhen, Money, Organization, Utc, util},
		clinvoice_data::Decimal,
		core::hash::Hash,
		std::{fs, time::Instant},
	};

	#[test]
	fn test_create()
	{
		fn assertion(bincode_job: BincodeJob<'_, '_, '_>)
		{
			let read_result = fs::read(bincode_job.filepath()).unwrap();
			assert_eq!(bincode_job.job, bincode::deserialize(&read_result).unwrap());
		}

		let start = Instant::now();

		let organization = Organization
		{
			id: Id::new_v4(),
			location_id: Id::new_v4(),
			name: "Big Old Test Corporation".into(),
			representatives: HashSet::new(),
		};

		util::test_temp_store(|store|
		{
			assertion(BincodeJob::create(
				organization.clone(),
				Utc::now(),
				Money::new(Decimal::new(200, 2), ""),
				"Test the job creation function.",
				*store,
			).unwrap());

			assertion(BincodeJob::create(
				organization.clone(),
				Utc::now(),
				Money::new(Decimal::new(200, 2), "USD"),
				"Test the job creation function.",
				*store,
			).unwrap());

			assertion(BincodeJob::create(
				organization.clone(),
				Utc::now(),
				Money::new(Decimal::new(20000, 0), "YEN"),
				"TEST THE JOB CREATION FUNCTION.",
				*store,
			).unwrap());

			assertion(BincodeJob::create(
				organization.clone(),
				Utc::now(),
				Money::new(Decimal::new(500, 2), "CDN"),
				"test the job creation function.",
				*store,
			).unwrap());

			assertion(BincodeJob::create(
				organization.clone(),
				Utc::now(),
				Money::new(Decimal::new(1000, 2), "EUR"),
				"TeSt ThE jOb CrEaTiOn FuNcTiOn.",
				*store,
			).unwrap());

			println!("\n>>>>> BincodeJob test_create {}us <<<<<\n", Instant::now().duration_since(start).as_micros());
		});
	}

	#[test]
	fn test_retrieve()
	{
		fn to_hashset<T>(slice: &[T]) -> HashSet<T> where T : Clone + Eq + Hash
		{
			return slice.iter().cloned().collect();
		}

		let start = Instant::now();

		let organization = Organization
		{
			id: Id::new_v4(),
			location_id: Id::new_v4(),
			name: "Big Old Test Corporation".into(),
			representatives: HashSet::new(),
		};

		util::test_temp_store(|store|
		{
			let creation = BincodeJob::create(
				organization.clone(),
				Utc::now(),
				Money::new(Decimal::new(200, 2), "USD"),
				"Test the job creation function.",
				*store,
			).unwrap();

			let retrieval = BincodeJob::create(
				organization.clone(),
				Utc::now(),
				Money::new(Decimal::new(200, 2), "USD"),
				"Test the job retrieval function.",
				*store,
			).unwrap();

			let assertion = BincodeJob::create(
				organization.clone(),
				Utc::now(),
				Money::new(Decimal::new(20000, 0), "YEN"),
				"Assert something",
				*store,
			).unwrap();

			// retrieve everything
			let mut results = BincodeJob::retrieve(
				MatchWhen::EqualTo(organization.id), // client
				MatchWhen::Any, // date close
				MatchWhen::Any, // date open
				MatchWhen::Any, // id
				MatchWhen::Any, // invoice date issued
				MatchWhen::Any, // invoice date paid
				MatchWhen::Any, // invoice hourly rate
				MatchWhen::Any, // notes
				MatchWhen::Any, // objectives
				MatchWhen::Any, // timesheet employee
				MatchWhen::Any, // timesheet time begin
				MatchWhen::Any, // timesheet time end
				*store,
			).unwrap();

			// assert the results are as expected
			assert!(results.contains(&creation));
			assert!(results.contains(&retrieval));
			assert!(results.contains(&assertion));

			// retrieve retrieval and assertion
			results = BincodeJob::retrieve(
				MatchWhen::Any, // client
				MatchWhen::Any, // date close
				MatchWhen::HasNone(to_hashset(&[creation.job.date_open])), // date open
				MatchWhen::HasAny(to_hashset(&[retrieval.job.id, assertion.job.id])), // id
				MatchWhen::Any, // invoice date issued
				MatchWhen::Any, // invoice date paid
				MatchWhen::Any, // invoice hourly rate
				MatchWhen::Any, // notes
				MatchWhen::Any, // objectives
				MatchWhen::Any, // timesheet employee
				MatchWhen::Any, // timesheet time begin
				MatchWhen::Any, // timesheet time end
				*store,
			).unwrap();

			// assert the results are as expected
			assert!(!results.contains(&creation));
			assert!(results.contains(&retrieval));
			assert!(results.contains(&assertion));

			println!("\n>>>>> BincodeJob test_retrieve {}us <<<<<\n", Instant::now().duration_since(start).as_micros());
		});
	}
}
