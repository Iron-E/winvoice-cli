use
{
	std::{borrow::Cow, error::Error},

	super::{Deletable, EmployeeAdapter, Initializable, LocationAdapter, query, Updatable},
	crate::Store,

	clinvoice_data::{Employee, Location, Organization, views::OrganizationView},
};

pub trait OrganizationAdapter  :
	Deletable<Error=<Self as OrganizationAdapter>::Error> +
	Initializable<Error=<Self as OrganizationAdapter>::Error> +
	Updatable<Error=<Self as OrganizationAdapter>::Error> +
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
	fn create(location: Location, name: &str, store: &Store) -> Result<Organization, <Self as OrganizationAdapter>::Error>;

	/// # Summary
	///
	/// Convert some `organization` into a [`OrganizationView`].
	fn into_view<L>(organization: Organization, store: &Store)
		-> Result<OrganizationView, <L as LocationAdapter>::Error>
	where
		L : LocationAdapter,
	{
		let location_result = Self::to_location::<L>(&organization, store)?;
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
		query: &query::Organization,
		store: &Store,
	) -> Result<Vec<Organization>, <Self as OrganizationAdapter>::Error>;

	/// # Summary
	///
	/// Get all of the [`Employee`]s which work at some `organization`.
	fn to_employees<E>(organization: &Organization, store: &Store)
		-> Result<Vec<Employee>, <E as EmployeeAdapter>::Error>
	where
		E : EmployeeAdapter,
	{
		E::retrieve(
			&query::Employee
			{
				organization: query::Organization
				{
					id: query::Match::EqualTo(Cow::Borrowed(&organization.id)),
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
	fn to_location<L>(organization: &Organization, store: &Store)
		-> Result<Location, <L as LocationAdapter>::Error>
	where
		L : LocationAdapter,
	{
		let results = L::retrieve(
			&query::Location
			{
				id: query::Match::EqualTo(Cow::Borrowed(&organization.location_id)),
				..Default::default()
			},
			store,
		)?;

		results.into_iter().next().ok_or_else(|| super::Error::DataIntegrity(organization.location_id).into())
	}
}
