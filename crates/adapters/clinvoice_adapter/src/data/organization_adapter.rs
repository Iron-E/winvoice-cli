use
{
	super::{Deletable, EmployeeAdapter, Initializable, LocationAdapter, Match, query, Updatable},
	crate::Store,
	clinvoice_data::{Employee, Location, Organization, views::OrganizationView},
	std::{borrow::Cow, error::Error},
};

pub trait OrganizationAdapter<'store>  :
	Deletable<Error=<Self as OrganizationAdapter<'store>>::Error> +
	Initializable<Error=<Self as OrganizationAdapter<'store>>::Error> +
	Updatable<Error=<Self as OrganizationAdapter<'store>>::Error> +
{
	type Error : From<super::Error> + Error;

	/// # Summary
	///
	/// Create a new [`Organization`] on the active [`Store`](crate::Store).
	///
	/// # Parameters
	///
	/// See [`Organization`].
	///
	/// # Returns
	///
	/// The newly created [`Organization`].
	fn create(location: Location, name: &str, store: &'store Store) -> Result<Organization, <Self as OrganizationAdapter<'store>>::Error>;

	/// # Summary
	///
	/// Get all of the [`Employee`]s which work at some `organization`.
	fn into_employees<E>(organization: &Organization, store: &'store Store)
		-> Result<Vec<Employee>, <E as EmployeeAdapter<'store>>::Error>
	where
		E : EmployeeAdapter<'store>,
	{
		E::retrieve(
			query::Employee
			{
				organization: query::Organization
				{
					id: Match::EqualTo(Cow::Borrowed(&organization.id)),
					..Default::default()
				},
				..Default::default()
			},
			store,
		)
	}

	/// # Summary
	///
	/// Convert some `organization` into a [`Location`] through it's `location_id` field.
	fn into_location<L>(organization: &Organization, store: &'store Store)
		-> Result<Location, <L as LocationAdapter<'store>>::Error>
	where
		L : LocationAdapter<'store>,
	{
		let results = L::retrieve(
			query::Location
			{
				id: Match::EqualTo(Cow::Borrowed(&organization.location_id)),
				..Default::default()
			},
			store,
		)?;

		let location = match results.get(0)
		{
			Some(loc) => loc,
			_ => return Err(super::Error::DataIntegrity(organization.location_id).into()),
		};

		Ok(location.clone())
	}

	/// # Summary
	///
	/// Convert some `organization` into a [`OrganizationView`].
	fn into_view<L>(organization: Organization, store: &'store Store)
		-> Result<OrganizationView, <L as LocationAdapter<'store>>::Error>
	where
		L : LocationAdapter<'store>,
	{
		let location_result = Self::into_location::<L>(&organization, store)?;
		let location_view_result = L::into_view(location_result, store);

		Ok(OrganizationView
		{
			id: organization.id,
			location: location_view_result?,
			name: organization.name,
		})
	}

	/// # Summary
	///
	/// Retrieve some [`Organization`] from the active [`Store`]crate::Store).
	///
	/// # Parameters
	///
	/// See [`Organization`].
	///
	/// # Returns
	///
	/// * An `Error`, if something goes wrong.
	/// * A list of matching [`Job`]s.
	fn retrieve(
		query: query::Organization,
		store: &Store,
	) -> Result<Vec<Organization>, <Self as OrganizationAdapter<'store>>::Error>;
}
