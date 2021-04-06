use
{
	super::{Deletable, EmployeeAdapter, Initializable, LocationAdapter, Match, OrganizationAdapter, PersonAdapter, query, timesheet, Updatable},
	crate::Store,
	clinvoice_data::
	{
		chrono::{DateTime, Utc},
		Job, Money, Organization,
		views::{JobView, TimesheetView},
	},
	std::{borrow::Cow, error::Error},
};

pub trait JobAdapter<'store> :
	Deletable<Error=<Self as JobAdapter<'store>>::Error> +
	Initializable<Error=<Self as JobAdapter<'store>>::Error> +
	Updatable<Error=<Self as JobAdapter<'store>>::Error> +
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
		store: &'store Store,
	) -> Result<Job, <Self as JobAdapter<'store>>::Error>;

	/// # Summary
	///
	/// Convert some `employee` into a [`Person`].
	fn into_organization<O>(job: &Job, store: &'store Store)
		-> Result<Organization, <O as OrganizationAdapter<'store>>::Error>
	where
		O : OrganizationAdapter<'store>,
	{
		let results = O::retrieve(
			query::Organization
			{
				id: Match::EqualTo(Cow::Borrowed(&job.client_id)),
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

	/// # Summary
	///
	/// Convert some `job` into a [`JobView`].
	fn into_view<E, L, O, P>(job: Job, store: &'store Store)
		-> Result<JobView, <Self as JobAdapter<'store>>::Error>
	where
		E : EmployeeAdapter<'store>,
		L : LocationAdapter<'store>,
		O : OrganizationAdapter<'store>,
		P : PersonAdapter<'store>,

		<E as EmployeeAdapter<'store>>::Error : From<<L as LocationAdapter<'store>>::Error>,
		<E as EmployeeAdapter<'store>>::Error : From<<O as OrganizationAdapter<'store>>::Error>,
		<E as EmployeeAdapter<'store>>::Error : From<<P as PersonAdapter<'store>>::Error>,

		<Self as JobAdapter<'store>>::Error : From<<E as EmployeeAdapter<'store>>::Error>,
	{
		let organization = Self::into_organization::<O>(&job, store).map_err(|e| e.into())?;
		let organization_view = O::into_view::<L>(organization, store).map_err(|e| e.into())?;

		let mut timesheet_views = Vec::with_capacity(job.timesheets.len());

		for timesheet in job.timesheets
		{
			let employee = timesheet::into_employee::<E>(&timesheet, store)?;
			let employee_view = E::into_view::<L, O, P>(employee, store)?;

			timesheet_views.push(TimesheetView
			{
				employee: employee_view,
				expenses: timesheet.expenses,
				time_begin: timesheet.time_begin,
				time_end: timesheet.time_end,
				work_notes: timesheet.work_notes,
			});
		}

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
		query: query::Job,
		store: &Store,
	) -> Result<Vec<Job>, <Self as JobAdapter<'store>>::Error>;
}
