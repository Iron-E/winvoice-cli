use
{
	std::{borrow::Cow::Borrowed, collections::HashMap, error::Error},

	super::{contact, Deletable, Initializable, LocationAdapter, OrganizationAdapter, PersonAdapter, Updatable},
	crate::Store,

	clinvoice_data::
	{
		Contact, Employee, EmployeeStatus, Organization, Person,
		views::{EmployeeView, PersonView},
	},
	clinvoice_query as query,
};

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
	fn create(
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
	fn into_view<L, O, P>(employee: Employee, store: &Store)
		-> Result<EmployeeView, <Self as EmployeeAdapter>::Error>
	where
		L : LocationAdapter,
		O : OrganizationAdapter,
		P : PersonAdapter,

		<Self as EmployeeAdapter>::Error :
			From<<L as LocationAdapter>::Error> +
			From<<O as OrganizationAdapter>::Error> +
			From<<P as PersonAdapter>::Error>,
	{
		let organization = Self::to_organization::<O>(&employee, store)?;
		let organization_view = O::into_view::<L>(organization, store)?;

		let person_view: PersonView = Self::to_person::<P>(&employee, store)?.into();

		let contact_info_view = contact::to_views::<L, String>(employee.contact_info, store)?;

		Ok(EmployeeView
		{
			contact_info: contact_info_view,
			id: employee.id,
			organization: organization_view,
			person: person_view,
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
	fn retrieve(
		query: &query::Employee,
		store: &Store,
	) -> Result<Vec<Employee>, <Self as EmployeeAdapter>::Error>;

	/// # Summary
	///
	/// Convert some `employee` into a [`Organization`].
	fn to_organization<O>(employee: &Employee, store: &Store)
		-> Result<Organization, <O as OrganizationAdapter>::Error>
	where
		O : OrganizationAdapter,
	{
		let results = O::retrieve(
			&query::Organization
			{
				id: query::Match::EqualTo(Borrowed(&employee.organization_id)),
				..Default::default()
			},
			store,
		)?;

		results.into_iter().next().ok_or_else(|| super::Error::DataIntegrity(employee.organization_id).into())
	}

	/// # Summary
	///
	/// Convert some `employee` into a [`Person`].
	fn to_person<P>(employee: &Employee, store: &Store)
		-> Result<Person, <P as PersonAdapter>::Error>
	where
		P : PersonAdapter,
	{
		let results = P::retrieve(
			&query::Person
			{
				id: query::Match::EqualTo(Borrowed(&employee.person_id)),
				..Default::default()
			},
			store,
		)?;

		results.into_iter().next().ok_or_else(|| super::Error::DataIntegrity(employee.organization_id).into())
	}
}
