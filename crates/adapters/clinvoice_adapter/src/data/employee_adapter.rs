#![allow(clippy::wrong_self_convention)]

use
{
	std::{borrow::Cow::Borrowed, collections::HashMap, error::Error, marker::Send},

	super::{contact, Deletable, Initializable, LocationAdapter, OrganizationAdapter, PersonAdapter, Updatable},
	crate::Store,

	clinvoice_data::{Contact, Employee, EmployeeStatus, Organization, Person, views::EmployeeView},
	clinvoice_query as query,

	async_trait::async_trait,
	futures::{FutureExt, TryFutureExt},
};

#[async_trait]
pub trait EmployeeAdapter :
	Deletable<Error=<Self as EmployeeAdapter>::Error> +
	Initializable<Error=<Self as EmployeeAdapter>::Error> +
	Updatable<Error=<Self as EmployeeAdapter>::Error> +
{
	type Error : From<super::Error> + Error;

	/// # Summary
	///
	/// Create some [`Employee`] on an active [`Store`](crate::Store).
	///
	/// # Parameters
	///
	/// See [`Employee`].
	///
	/// # Returns
	///
	/// * The created [`Employee`], if there were no errors.
	/// * An [`Error`], if something goes wrong.
	async fn create(
		contact_info: HashMap<String, Contact>,
		organization: Organization,
		person: Person,
		status: EmployeeStatus,
		title: String,
		store: &Store,
	) -> Result<Employee, <Self as EmployeeAdapter>::Error>;

	/// # Summary
	///
	/// Convert some `employee` into a [`EmployeeView`].
	async fn into_view<L, O, P>(employee: Employee, store: &Store)
		-> Result<EmployeeView, <Self as EmployeeAdapter>::Error>
	where
		L : LocationAdapter + Send,
		O : OrganizationAdapter + Send,
		P : PersonAdapter,

		<L as LocationAdapter>::Error : Send,
		<Self as EmployeeAdapter>::Error :
			From<<L as LocationAdapter>::Error> +
			From<<O as OrganizationAdapter>::Error> +
			From<<P as PersonAdapter>::Error> +
			Send,
	{
		let organization_view = Self::to_organization::<O>(&employee, store).map_err(|e|
			<Self as EmployeeAdapter>::Error::from(e)
		).and_then(|organization| O::into_view::<L>(organization, store).err_into());

		let person_view = Self::to_person::<P>(&employee, store);

		let contact_info_view = contact::to_views::<L, String>(employee.contact_info.clone(), store);

		Ok(EmployeeView
		{
			contact_info: contact_info_view.await?,
			id: employee.id,
			organization: organization_view.await?,
			person: person_view.await?.into(),
			status: employee.status,
			title: employee.title,
		})
	}

	/// # Summary
	///
	/// Retrieve some [`Employee`] from an active [`Store`](crate::Store).
	///
	/// # Parameters
	///
	/// See [`Employee`].
	///
	/// # Returns
	///
	/// * Any matching [`Employee`]s.
	/// * An [`Error`], should something go wrong.
	async fn retrieve(
		query: &query::Employee,
		store: &Store,
	) -> Result<Vec<Employee>, <Self as EmployeeAdapter>::Error>;

	/// # Summary
	///
	/// Convert some `employee` into a [`Organization`].
	async fn to_organization<O>(employee: &Employee, store: &Store)
		-> Result<Organization, <O as OrganizationAdapter>::Error>
	where
		O : OrganizationAdapter,
	{
		let query = query::Organization
		{
			id: query::Match::EqualTo(Borrowed(&employee.organization_id)),
			..Default::default()
		};

		O::retrieve(&query, store).map(|result| result.and_then(|retrieved|
			retrieved.into_iter().next().ok_or_else(|| super::Error::DataIntegrity(employee.organization_id).into())
		)).await
	}

	/// # Summary
	///
	/// Convert some `employee` into a [`Person`].
	async fn to_person<P>(employee: &Employee, store: &Store)
		-> Result<Person, <P as PersonAdapter>::Error>
	where
		P : PersonAdapter,
	{
		let query = query::Person
		{
			id: query::Match::EqualTo(Borrowed(&employee.person_id)),
			..Default::default()
		};

		P::retrieve(&query, store).map(|result| result.and_then(|retrieved|
			retrieved.into_iter().next().ok_or_else(|| super::Error::DataIntegrity(employee.organization_id).into())
		)).await
	}
}
