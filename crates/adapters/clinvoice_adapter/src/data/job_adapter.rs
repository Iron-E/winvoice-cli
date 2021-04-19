use
{
	std::{borrow::Cow, error::Error},

	super::{Deletable, EmployeeAdapter, Initializable, LocationAdapter, OrganizationAdapter, PersonAdapter, query, timesheet, Updatable},
	crate::Store,

	clinvoice_data::
	{
		chrono::{DateTime, Utc},
		Job, Money, Organization,
		views::{JobView, TimesheetView},
	},
};

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
	fn create(
		client: Organization,
		date_open: DateTime<Utc>,
		hourly_rate: Money,
		objectives: &str,
		store: &Store,
	) -> Result<Job, <Self as JobAdapter>::Error>;

	/// # Summary
	///
	/// Convert some `job` into a [`JobView`].
	fn into_view<E, L, O, P>(job: Job, store: &Store)
		-> Result<JobView, <Self as JobAdapter>::Error>
	where
		E : EmployeeAdapter,
		L : LocationAdapter,
		O : OrganizationAdapter,
		P : PersonAdapter,

		<E as EmployeeAdapter>::Error : From<<L as LocationAdapter>::Error>,
		<E as EmployeeAdapter>::Error : From<<O as OrganizationAdapter>::Error>,
		<E as EmployeeAdapter>::Error : From<<P as PersonAdapter>::Error>,

		<Self as JobAdapter>::Error : From<<E as EmployeeAdapter>::Error>,
	{
		let organization = Self::to_organization::<O>(&job, store).map_err(|e| e.into())?;
		let organization_view = O::into_view::<L>(organization, store).map_err(|e| e.into())?;

		let timesheets_len = job.timesheets.len();
		let timesheet_views = job.timesheets.into_iter().try_fold(
			Vec::with_capacity(timesheets_len),
			|mut v, t| -> Result<_, <E as EmployeeAdapter>::Error>
			{
				let employee = timesheet::to_employee::<E>(&t, store)?;
				let employee_view = E::into_view::<L, O, P>(employee, store)?;

				v.push(TimesheetView
				{
					employee: employee_view,
					expenses: t.expenses,
					time_begin: t.time_begin,
					time_end: t.time_end,
					work_notes: t.work_notes,
				});

				Ok(v)
			},
		)?;

		Ok(JobView
		{
			client: organization_view,
			date_close: job.date_close,
			date_open: job.date_open,
			id: job.id,
			invoice: job.invoice,
			notes: job.notes,
			objectives: job.objectives,
			timesheets: timesheet_views,
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
	fn retrieve(
		query: &query::Job,
		store: &Store,
	) -> Result<Vec<Job>, <Self as JobAdapter>::Error>;

	/// # Summary
	///
	/// Convert some `employee` into a [`Person`].
	fn to_organization<O>(job: &Job, store: &Store)
		-> Result<Organization, <O as OrganizationAdapter>::Error>
	where
		O : OrganizationAdapter,
	{
		let results = O::retrieve(
			&query::Organization
			{
				id: query::Match::EqualTo(Cow::Borrowed(&job.client_id)),
				..Default::default()
			},
			store,
		)?;

		let organization = match results.get(0)
		{
			Some(org) => org,
			_ => return Err(super::Error::DataIntegrity(job.client_id).into()),
		};

		Ok(organization.clone())
	}
}
