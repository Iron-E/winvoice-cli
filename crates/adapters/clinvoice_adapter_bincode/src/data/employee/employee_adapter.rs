use
{
	std::collections::HashMap,

	super::BincodeEmployee,
	crate::
	{
		data::{Error, Result},
		util,
	},

	clinvoice_adapter::
	{
		data::{EmployeeAdapter, Error as DataError, Initializable, Updatable},
		Store,
	},
	clinvoice_data::{Contact, Employee, EmployeeStatus, Organization, Person},
	clinvoice_query as query,
};

#[async_trait::async_trait]
impl EmployeeAdapter for BincodeEmployee<'_, '_>
{
	type Error = Error;

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
	) -> Result<Employee>
	{
		let init_fut = Self::init(&store);

		let employee = Employee
		{
			contact_info,
			id: util::unique_id(&Self::path(&store))?,
			organization_id: organization.id,
			person_id: person.id,
			title,
			status,
		};

		init_fut.await?;
		BincodeEmployee {employee: &employee, store}.update().await?;

		Ok(employee)
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
	async fn retrieve(query: &query::Employee, store: &Store) -> Result<Vec<Employee>>
	{
		Self::init(&store).await?;

		util::retrieve(Self::path(store),
			|e| query.matches(e).map_err(|e| DataError::from(e).into())
		).await
	}
}

#[cfg(test)]
mod tests
{
	use
	{
		std::{borrow::Cow::Borrowed, time::Instant},

		super::{BincodeEmployee, Contact, Employee, EmployeeAdapter, EmployeeStatus, Organization, Person, query, Store, util},

		clinvoice_data::Id,
		clinvoice_query::Match,

		tokio::fs,
	};

	#[tokio::test]
	async fn create()
	{
		let organization = Organization
		{
			id: Id::new_v4(),
			location_id: Id::new_v4(),
			name: "Big Old Test Corporation".into(),
		};

		util::temp_store(|store| async move
		{
			let start = Instant::now();

			let (testy, nimron, tortust, gottard, duplicate) = futures::try_join!(
				BincodeEmployee::create(
					vec![("Work".into(), Contact::Address {location_id: Id::new_v4(), export: false})].into_iter().collect(),
					organization.clone(),
					Person
					{
						id: Id::new_v4(),
						name: "Testy Mćtesterson".into(),
					},
					EmployeeStatus::Employed,
					"CEO of Tests".into(),
					&store,
				),

				BincodeEmployee::create(
					vec![("Work Email".into(), Contact::Email {email: "foo@bar.io".into(), export: false})].into_iter().collect(),
					organization.clone(),
					Person
					{
						id: Id::new_v4(),
						name: "Nimron MacBeaver".into(),
					},
					EmployeeStatus::NotEmployed,
					"Oblong Shape Holder".into(),
					&store,
				),

				BincodeEmployee::create(
					vec![("Work Phone".into(), Contact::Phone {phone: "1-800-555-3600".into(), export: false})].into_iter().collect(),
					organization.clone(),
					Person
					{
						id: Id::new_v4(),
						name: "An Actual «Tor♯tust".into(),
					},
					EmployeeStatus::Representative,
					"Mixer of Soups".into(),
					&store,
				),

				BincodeEmployee::create(
					vec![("Work".into(), Contact::Address {location_id: Id::new_v4(), export: false})].into_iter().collect(),
					organization.clone(),
					Person
					{
						id: Id::new_v4(),
						name: "Jimmy Neutron, Boy Genius' Dog 'Gottard'".into(),
					},
					EmployeeStatus::Employed,
					"Sidekick".into(),
					&store,
				),

				BincodeEmployee::create(
					vec![("Work Email".into(), Contact::Email {email: "obviousemail@server.com".into(), export: false})].into_iter().collect(),
					organization.clone(),
					Person
					{
						id: Id::new_v4(),
						name: "Testy Mćtesterson".into(),
					},
					EmployeeStatus::NotEmployed,
					"Lazy No-good Duplicate Name User".into(),
					&store,
				),
			).unwrap();

			println!("\n>>>>> BincodeEmployee::create {}us <<<<<\n", Instant::now().duration_since(start).as_micros() / 5);

			futures::join!(
				create_assertion(testy, store),
				create_assertion(nimron, store),
				create_assertion(tortust, store),
				create_assertion(gottard, store),
				create_assertion(duplicate, store),
			);
		}).await;
	}

	async fn create_assertion(employee: Employee, store: &Store)
	{
		let read_result = fs::read(BincodeEmployee {employee: &employee, store}.filepath()).await.unwrap();
		assert_eq!(employee, bincode::deserialize(&read_result).unwrap());
	}

	#[tokio::test]
	async fn retrieve()
	{
		let organization = Organization
		{
			id: Id::new_v4(),
			location_id: Id::new_v4(),
			name: "Big Old Test Corporation".into(),
		};

		util::temp_store(|store| async move
		{
			let (testy, nimron, tortust, gottard, duplicate) = futures::try_join!(
				BincodeEmployee::create(
					vec![("Work Address".into(), Contact::Address {location_id: Id::new_v4(), export: false})].into_iter().collect(),
					organization.clone(),
					Person
					{
						id: Id::new_v4(),
						name: "Testy Mćtesterson".into(),
					},
					EmployeeStatus::NotEmployed,
					"CEO of Tests".into(),
					&store,
				),

				BincodeEmployee::create(
					vec![("Home Address".into(), Contact::Email {email: "foo@bar.io".into(), export: false})].into_iter().collect(),
					organization.clone(),
					Person
					{
						id: Id::new_v4(),
						name: "Nimron MacBeaver".into(),
					},
					EmployeeStatus::Employed,
					"Oblong Shape Holder".into(),
					&store,
				),

				BincodeEmployee::create(
					vec![("Work Phone".into(), Contact::Phone {phone: "1-800-555-3600".into(), export: false})].into_iter().collect(),
					organization.clone(),
					Person
					{
						id: Id::new_v4(),
						name: "An Actual «Tor♯tust".into(),
					},
					EmployeeStatus::Representative,
					"Mixer of Soups".into(),
					&store,
				),

				BincodeEmployee::create(
					vec![("Work Address".into(), Contact::Address {location_id: Id::new_v4(), export: false})].into_iter().collect(),
					organization.clone(),
					Person
					{
						id: Id::new_v4(),
						name: "Jimmy Neutron, Boy Genius' Dog 'Gottard'".into(),
					},
					EmployeeStatus::Employed,
					"Sidekick".into(),
					&store,
				),

				BincodeEmployee::create(
					vec![("Work Email".into(), Contact::Email {email: "obviousemail@server.com".into(), export: false})].into_iter().collect(),
					organization.clone(),
					Person
					{
						id: Id::new_v4(),
						name: "Testy Mćtesterson".into(),
					},
					EmployeeStatus::NotEmployed,
					"Lazy No-good Duplicate Name User".into(),
					&store,
				),
			).unwrap();

			let start = Instant::now();

			let (everything, testy_gottard) = futures::try_join!(
				// retrieve everything
				BincodeEmployee::retrieve(&Default::default(), &store),

				// Retrieve testy and gottard
				BincodeEmployee::retrieve(
					&query::Employee
					{
						id: Match::HasAny(vec![Borrowed(&testy.id), Borrowed(&gottard.id)].into_iter().collect()),
						..Default::default()
					},
					&store,
				),
			).unwrap();

			println!("\n>>>>> BincodeEmployee::retrieve {}us <<<<<\n", Instant::now().duration_since(start).as_micros() / 2);

			// Assert the results contains all values
			assert!(everything.contains(&tortust));
			assert!(everything.contains(&duplicate));
			assert!(everything.contains(&gottard));
			assert!(everything.contains(&nimron));
			assert!(everything.contains(&testy));

			// Assert the results contains all expected values
			assert!(!testy_gottard.contains(&tortust));
			assert!(!testy_gottard.contains(&duplicate));
			assert!(testy_gottard.contains(&gottard));
			assert!(!testy_gottard.contains(&nimron));
			assert!(testy_gottard.contains(&testy));
		});
	}
}
