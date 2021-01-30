use super::{Deletable, RetrieveWhen, Updatable};
use crate::Store;
use clinvoice_data::{chrono::{DateTime, Utc}, Job, Money, Organization, Id};
use std::{collections::HashSet, error::Error};

pub trait JobAdapter<'currency, 'objectives, 'name, 'notes, 'pass, 'path, 'title, 'user, 'work_notes> :
	Deletable<'pass, 'path, 'user> +
	Into<Job<'currency, 'objectives, 'notes, 'work_notes>> +
	Into<Result<Organization<'name>, Box<dyn Error>>> +
	Into<Store<'pass, 'path, 'user>> +
	Updatable +
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
	) -> Result<Self, Box<dyn Error>>;

	/// # Summary
	///
	/// Initialize the database for a given [`Store`].
	fn init(store: &Store<'pass, 'path, 'user>) -> Result<(), Box<dyn Error>>;

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
		client: RetrieveWhen<Organization<'name>>,
		date_close: RetrieveWhen<Option<DateTime<Utc>>>,
		date_open: RetrieveWhen<DateTime<Utc>>,
		id: RetrieveWhen<Id>,
		invoice_date_issued: RetrieveWhen<DateTime<Utc>>,
		invoice_date_paid: RetrieveWhen<DateTime<Utc>>,
		invoice_hourly_rate: RetrieveWhen<Money<'currency>>,
		notes: RetrieveWhen<&'notes str>,
		objectives: RetrieveWhen<&'objectives str>,
		store: Store<'pass, 'path, 'user>,
	) -> Result<HashSet<Self>, Box<dyn Error>>;
}
