use
{
	super::BincodeEmployee,
	crate::data::{BincodeOrganization, BincodePerson, contact, Result},
	clinvoice_data::
	{
		Organization, Person,
		views::{EmployeeView, OrganizationView, PersonView}
	},
};

impl Into<Result<EmployeeView>> for BincodeEmployee<'_, '_>
{
	fn into(self) -> Result<EmployeeView>

	{
		let id = self.employee.id;
		let status = self.employee.status;
		let store = self.store;
		let title = self.employee.title.clone();

		let contact_info_view = contact::into_views(self.employee.contact_info.clone(), self.store)?;

		let organization_result: Result<Organization> = self.clone().into();
		let organization_view_result: Result<OrganizationView> = BincodeOrganization
		{
			organization: &organization_result?,
			store,
		}.into();

		let person_result: Result<Person> = self.into();
		let person_view_result: Result<PersonView> = BincodePerson
		{
			person: &person_result?,
			store,
		}.into();

		Ok(EmployeeView
		{
			contact_info: contact_info_view,
			id,
			organization: organization_view_result?,
			person: person_view_result?,
			status,
			title,
		})
	}
}

#[cfg(test)]
mod tests
{
	use
	{
		super::{BincodeEmployee, EmployeeView, OrganizationView, PersonView, Result},
		crate::
		{
			data::{BincodeLocation, BincodeOrganization, BincodePerson},
			util,
		},
		clinvoice_adapter::data::{EmployeeAdapter, LocationAdapter, OrganizationAdapter, PersonAdapter},
		clinvoice_data::
		{
			Contact, EmployeeStatus,
			views::{ContactView, LocationView},
		},
		std::time::Instant,
	};

	#[test]
	fn test_into_view()
	{
		util::test_temp_store(|store|
		{
			let earth = BincodeLocation::create("Earth", &store).unwrap();

			let big_old_test = BincodeOrganization::create(
				earth.clone(),
				"Big Old Test Corporation",
				&store,
			).unwrap();

			let testy = BincodePerson::create(
				"Testy Mćtesterson",
				&store,
			).unwrap();

			let ceo_testy = BincodeEmployee::create(
				vec![("Work".into(), Contact::Address(earth.id))].into_iter().collect(),
				big_old_test.clone(),
				testy.clone(),
				EmployeeStatus::Employed,
				"CEO of Tests",
				&store,
			).unwrap();

			let earth_view = LocationView
			{
				id: earth.id,
				name: earth.name,
				outer: None,
			};

			let ceo_testy_view = EmployeeView
			{
				contact_info: vec![("Work".into(), ContactView::Address(earth_view.clone()))].into_iter().collect(),
				id: ceo_testy.id,
				organization: OrganizationView
				{
					id: big_old_test.id,
					location: earth_view.clone(),
					name: big_old_test.name,
				},
				person: PersonView
				{
					id: testy.id,
					name: testy.name,
				},
				title: ceo_testy.title.clone(),
				status: ceo_testy.status,
			};

			let start = Instant::now();
			let ceo_testy_view_result: Result<EmployeeView> = BincodeEmployee {employee: &ceo_testy, store}.into();
			println!("\n>>>>> BincodeEmployee::into_view {}us <<<<<\n", Instant::now().duration_since(start).as_micros());

			// Asser that the synthetic view is the same as the view which was created naturally.
			assert_eq!(ceo_testy_view, ceo_testy_view_result.unwrap());
		});
	}
}
