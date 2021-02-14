use
{
	super::{Deletable, Initializable, MatchWhen, Updatable},
	crate::Store,
	clinvoice_data::
	{
		chrono::{DateTime, Utc},
		Id, Job, Money, Organization
	},
	std::{collections::HashSet, error::Error},
};

pub trait JobAdapter<'pass, 'path, 'user> :
	Deletable +
	Initializable<'pass, 'path, 'user> +
	Into<Job> +
	Into<Result<Organization, Box<dyn Error>>> +
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
	fn create<'objectives>(
		client: Organization,
		date_open: DateTime<Utc>,
		hourly_rate: Money,
		objectives: &'objectives str,
		store: Store<'pass, 'path, 'user>,
	) -> Result<Self, Box<dyn Error>>;

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
	) -> Result<HashSet<Self>, Box<dyn Error>>;
}
