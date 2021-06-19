#![allow(clippy::wrong_self_convention)]

use
{
	std::{borrow::Cow::Borrowed, error::Error, marker::Send},

	super::{Deletable, EmployeeAdapter, Initializable, LocationAdapter, Updatable},
	crate::Store,

	clinvoice_data::{Employee, Location, Organization, views::OrganizationView},
	clinvoice_query as query,

	async_trait::async_trait,
	futures::{FutureExt, TryFutureExt},
};

#[async_trait]
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
	async fn create(location: Location, name: String, store: &Store) -> Result<Organization, <Self as OrganizationAdapter>::Error>;

	/// # Summary
	///
	/// Convert some `organization` into a [`OrganizationView`].
	async fn into_view<L>(organization: Organization, store: &Store)
		-> Result<OrganizationView, <L as LocationAdapter>::Error>
	where
		L : LocationAdapter + Send,
	{
		let location_view = Self::to_location::<L>(&organization, store).and_then(|result| L::into_view(result, store));

		Ok(OrganizationView
		{
			id: organization.id,
			location: location_view.await?,
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
	async fn retrieve(
		query: &query::Organization,
		store: &Store,
	) -> Result<Vec<Organization>, <Self as OrganizationAdapter>::Error>;

	/// # Summary
	///
	/// Get all of the [`Employee`]s which work at some `organization`.
	async fn to_employees<E>(organization: &Organization, store: &Store)
		-> Result<Vec<Employee>, <E as EmployeeAdapter>::Error>
	where
		E : EmployeeAdapter,
	{
		E::retrieve(
			&query::Employee
			{
				organization: query::Organization
				{
					id: query::Match::EqualTo(Borrowed(&organization.id)),
					..Default::default()
				},
				..Default::default()
			},
			store,
		).await
	}

	/// # Summary
	///
	/// Convert some `organization` into a [`Location`] through it's `location_id` field.
	async fn to_location<L>(organization: &Organization, store: &Store)
		-> Result<Location, <L as LocationAdapter>::Error>
	where
		L : LocationAdapter,
	{
		let query = query::Location
		{
			id: query::Match::EqualTo(Borrowed(&organization.location_id)),
			..Default::default()
		};

		L::retrieve(&query, store).map(|result| result.and_then(|retrieved|
			retrieved.into_iter().next().ok_or_else(|| super::Error::DataIntegrity(organization.location_id).into())
		)).await
	}
}
