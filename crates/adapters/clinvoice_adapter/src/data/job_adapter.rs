use
{
	super::{Deletable, Initializable, MatchWhen, Updatable},
	crate::{DynamicResult, Store},
	clinvoice_data::
	{
		chrono::{DateTime, Utc},
		Id, InvoiceDate, Job, Money, Organization, views::JobView
	},
};

pub trait JobAdapter<'pass, 'path, 'user> :
	Deletable +
	Initializable +
	Into<Job> +
	Into<DynamicResult<JobView>> +
	Into<DynamicResult<Organization>> +
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
	) -> DynamicResult<Self>;

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
		store: Store<'pass, 'path, 'user>,
	) -> DynamicResult<Vec<Self>>;
}
