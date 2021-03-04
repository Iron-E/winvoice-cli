use
{
	super::{Deletable, Initializable, MatchWhen, Updatable},
	crate::Store,
	clinvoice_data::
	{
		chrono::{DateTime, Utc},
		Id, InvoiceDate, Job, Money, Organization, views::JobView
	},
	std::error::Error,
};

pub trait JobAdapter :
	Deletable<Error=<Self as JobAdapter>::Error> +
	Initializable<Error=<Self as JobAdapter>::Error> +
	Into<Job> +
	Into<Result<JobView, <Self as JobAdapter>::Error>> +
	Into<Result<Organization, <Self as JobAdapter>::Error>> +
	Updatable<Error=<Self as JobAdapter>::Error> +
{ type Error : Error;

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
		store: Store,
	) -> Result<Job, <Self as JobAdapter>::Error>;

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
		store: Store,
	) -> Result<Vec<Job>, <Self as JobAdapter>::Error>;
}
