use
{
	super::{contact, Deletable, Initializable, LocationAdapter, Match, OrganizationAdapter, PersonAdapter, query, Updatable},
	crate::Store,
	clinvoice_data::
	{
		Contact, Employee, EmployeeStatus, Organization, Person,
		views::{EmployeeView, PersonView},
	},
	std::{borrow::Cow, collections::HashMap, error::Error},
};

pub trait EmployeeAdapter<'store> :
	Deletable<Error=<Self as EmployeeAdapter<'store>>::Error> +
	Initializable<Error=<Self as EmployeeAdapter<'store>>::Error> +
	Updatable<Error=<Self as EmployeeAdapter<'store>>::Error> +
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
		title: &str,
		store: &'store Store,
	) -> Result<Employee, <Self as EmployeeAdapter<'store>>::Error>;

	/// # Summary
	///
	/// Convert some `employee` into a [`Organization`].
	fn into_organization<O>(employee: &Employee, store: &'store Store)
		-> Result<Organization, <O as OrganizationAdapter<'store>>::Error>
	where
		O : OrganizationAdapter<'store>,
	{
		let results = O::retrieve(
			query::Organization
			{
				id: Match::EqualTo(Cow::Borrowed(&employee.organization_id)),
				..Default::default()
			},
			store,
		)?;

		let organization = match results.get(0)
		{
			Some(org) => org,
			_ => return Err(super::Error::DataIntegrity {id: employee.organization_id}.into()),
		};

		Ok(organization.clone())
	}

	/// # Summary
	///
	/// Convert some `employee` into a [`Person`].
	fn into_person<P>(employee: &Employee, store: &'store Store)
		-> Result<Person, <P as PersonAdapter<'store>>::Error>
	where
		P : PersonAdapter<'store>,
	{
		let results = P::retrieve(
			query::Person
			{
				id: Match::EqualTo(Cow::Borrowed(&employee.person_id)),
				..Default::default()
			},
			store,
		)?;

		let person = match results.get(0)
		{
			Some(org) => org,
			_ => return Err(super::Error::DataIntegrity {id: employee.organization_id}.into()),
		};

		Ok(person.clone())
	}

	/// # Summary
	///
	/// Convert some `employee` into a [`EmployeeView`].
	fn into_view<L, O, P>(employee: Employee, store: &'store Store)
		-> Result<EmployeeView, <Self as EmployeeAdapter<'store>>::Error>
	where
		L : LocationAdapter<'store>,
		O : OrganizationAdapter<'store>,
		P : PersonAdapter<'store>,

		<Self as EmployeeAdapter<'store>>::Error : From<<L as LocationAdapter<'store>>::Error>,
		<Self as EmployeeAdapter<'store>>::Error : From<<O as OrganizationAdapter<'store>>::Error>,
		<Self as EmployeeAdapter<'store>>::Error : From<<P as PersonAdapter<'store>>::Error>,
	{
		let organization = Self::into_organization::<O>(&employee, store)?;
		let organization_view = O::into_view::<L>(organization, store)?;

		let person_view: PersonView = Self::into_person::<P>(&employee, store)?.into();

		let contact_info_view = contact::into_views::<L, String>(employee.contact_info, store)?;

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
		query: query::Employee,
		store: &Store,
	) -> Result<Vec<Employee>, <Self as EmployeeAdapter<'store>>::Error>;
}
