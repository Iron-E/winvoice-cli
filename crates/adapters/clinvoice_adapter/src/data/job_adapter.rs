#![allow(clippy::wrong_self_convention)]

use
{
	std::{borrow::Cow::Borrowed, error::Error, marker::Send},

	super::{Deletable, EmployeeAdapter, Initializable, LocationAdapter, OrganizationAdapter, PersonAdapter, timesheet, Updatable},
	crate::Store,

	clinvoice_data::
	{
		chrono::{DateTime, Utc},
		Job, finance::Money, Organization,
		views::{JobView, TimesheetView},
	},
	clinvoice_query as query,

	futures::
	{
		FutureExt,
		stream::{self, TryStreamExt},
		TryFutureExt,
	},
};

#[async_trait::async_trait]
pub trait JobAdapter :
	Deletable<Error=<Self as JobAdapter>::Error> +
	Initializable<Error=<Self as JobAdapter>::Error> +
	Updatable<Error=<Self as JobAdapter>::Error> +
{
	type Error : From<super::Error> + Error;

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
	) -> Result<Job, <Self as JobAdapter>::Error>;

	/// # Summary
	///
	/// Convert some `job` into a [`JobView`].
	async fn into_view<E, L, O, P>(job: Job, store: &Store)
		-> Result<JobView, <Self as JobAdapter>::Error>
	where
		E : EmployeeAdapter + Send,
		L : LocationAdapter + Send,
		O : OrganizationAdapter + Send,
		P : PersonAdapter,

		<E as EmployeeAdapter>::Error :
			From<<L as LocationAdapter>::Error> +
			From<<O as OrganizationAdapter>::Error> +
			From<<P as PersonAdapter>::Error> +
			Send,
		<L as LocationAdapter>::Error : Send,
		<Self as JobAdapter>::Error : From<<E as EmployeeAdapter>::Error>,
	{
		let organization_view = Self::to_organization::<O>(&job, store).err_into().and_then(|organization|
			O::into_view::<L>(organization, store).err_into()
		);

		let timesheet_views = stream::iter(job.timesheets.iter().map(|t| Ok(t))).and_then(|t|
			timesheet::to_employee::<E>(&t, store).and_then(|employee|
				E::into_view::<L, O, P>(employee, store)
			).map_ok(move |employee_view|
				TimesheetView
				{
					employee: employee_view,
					expenses: t.expenses.clone(),
					time_begin: t.time_begin,
					time_end: t.time_end,
					work_notes: t.work_notes.clone(),
				}
			)
		).try_collect();

		Ok(JobView
		{
			client: organization_view.await?,
			date_close: job.date_close,
			date_open: job.date_open,
			id: job.id,
			invoice: job.invoice,
			notes: job.notes,
			objectives: job.objectives,
			timesheets: timesheet_views.await?,
		})
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
	async fn retrieve(
		query: &query::Job,
		store: &Store,
	) -> Result<Vec<Job>, <Self as JobAdapter>::Error>;

	/// # Summary
	///
	/// Convert some `employee` into a [`Person`].
	async fn to_organization<O>(job: &Job, store: &Store)
		-> Result<Organization, <O as OrganizationAdapter>::Error>
	where
		O : OrganizationAdapter + Send,
	{
		let query = query::Organization
		{
			id: query::Match::EqualTo(Borrowed(&job.client_id)),
			..Default::default()
		};

		O::retrieve(&query, store).map(|result| result.and_then(|retrieved|
			retrieved.into_iter().next().ok_or_else(|| super::Error::DataIntegrity(job.client_id).into())
		)).await
	}
}
