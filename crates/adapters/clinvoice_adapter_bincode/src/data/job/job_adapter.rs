use super::BincodeJob;
use crate::util;
use clinvoice_adapter::{data::{AnyValue, JobAdapter, Updatable}, Store};
use clinvoice_data::{chrono::{DateTime, Utc}, Invoice, Job, Money, Organization, Id};
use std::{collections::{BTreeSet, HashSet}, error::Error};

impl<'currency, 'objectives, 'name, 'notes, 'pass, 'path, 'title, 'user, 'work_notes> JobAdapter<'currency, 'objectives, 'name, 'notes, 'pass, 'path, 'title, 'user, 'work_notes>
for BincodeJob<'currency, 'objectives, 'notes, 'work_notes, 'pass, 'path, 'user>
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
	fn create(
		client: Organization<'name>,
		date_open: DateTime<Utc>,
		hourly_rate: Money<'currency>,
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
				objectives,
				notes: "",
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
		client: AnyValue<Organization<'name>>,
		date_close: AnyValue<DateTime<Utc>>,
		date_open: AnyValue<DateTime<Utc>>,
		id: AnyValue<Id>,
		invoice_date_issued: AnyValue<DateTime<Utc>>,
		invoice_date_paid: AnyValue<DateTime<Utc>>,
		invoice_hourly_rate: AnyValue<Money<'currency>>,
		notes: AnyValue<&'notes str>,
		objectives: AnyValue<&'objectives str>,
		store: Store<'pass, 'path, 'user>,
	) -> Result<HashSet<Self>, Box<dyn Error>>
	{
		todo!()
	}
}

#[cfg(test)]
mod tests
{
	use super::{BincodeJob, Id, HashSet, JobAdapter, Money, Organization, Utc, util};
	use clinvoice_data::Decimal;
	use std::{fs, io};

	#[test]
	fn test_create() -> Result<(), io::Error>
	{
		fn assertion(bincode_job: BincodeJob<'_, '_, '_, '_, '_, '_, '_>)
		{
			let read_result = fs::read(bincode_job.filepath()).unwrap();

			assert_eq!(bincode_job.job, bincode::deserialize(&read_result).unwrap());
		}

		let organization = Organization
		{
			id: Id::new_v4(),
			location_id: Id::new_v4(),
			name: "Big Old Test Corporation",
			representatives: HashSet::new(),
		};

		return util::test_temp_store(|store|
		{
			assertion(BincodeJob::create(
				organization.clone(),
				Utc::now(),
				Money {amount: Decimal::new(200, 2), currency: "".into()},
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

			assert!(fs::remove_dir_all(BincodeJob::path(&store)).is_ok());
		});
	}

	#[test]
	fn test_init() -> Result<(), io::Error>
	{
		return util::test_temp_store(|store|
		{
			// Assert that the function can initialize the store.
			assert!(BincodeJob::init(store).is_ok());

			// Create filepath for temporary test file.
			let filepath = BincodeJob::path(store).join("testfile.txt");

			// Assert that creation of a file inside the initialized space is done
			assert!(fs::write(&filepath, "").is_ok());

			// Assert that the function will still return OK with files in the directory.
			assert!(BincodeJob::init(store).is_ok());

			// Assert cleanup
			assert!(fs::remove_file(filepath).is_ok());
		});
	}
}
