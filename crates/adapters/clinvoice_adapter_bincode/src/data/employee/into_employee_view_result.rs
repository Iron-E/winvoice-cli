use
{
	super::BincodeEmployee,
	crate::data::{BincodeOrganization, BincodePerson, contact},
	clinvoice_adapter::DynamicResult,
	clinvoice_data::
	{
		Organization, Person,
		views::{EmployeeView, OrganizationView, PersonView}
	},
};

impl Into<DynamicResult<EmployeeView>> for BincodeEmployee<'_, '_, '_>
{
	fn into(self) -> DynamicResult<EmployeeView>

	{
		let contact_info_view = contact::into_views(self.employee.contact_info.clone(), self.store)?;

		let organization_result: DynamicResult<Organization> = self.clone().into();
		let organization_view_result: DynamicResult<OrganizationView> = BincodeOrganization
		{
			organization: organization_result?,
			store: self.store,
		}.into();

		let person_result: DynamicResult<Person> = self.clone().into();
		let person_view_result: DynamicResult<PersonView> = BincodePerson
		{
			person: person_result?,
			store: self.store,
		}.into();

		return Ok(EmployeeView
		{
			contact_info: contact_info_view,
			organization: organization_view_result?,
			person: person_view_result?,
			status: self.employee.status,
			title: self.employee.title,
		});
	}
}

#[cfg(test)]
mod tests
{
	use
	{
		super::{BincodeEmployee, EmployeeView, OrganizationView, PersonView},
		crate::
		{
			data::{BincodeLocation, BincodeOrganization, BincodePerson},
			util,
		},
		clinvoice_adapter::
		{
			DynamicResult,
			data::{EmployeeAdapter, LocationAdapter, OrganizationAdapter, PersonAdapter}
		},
		clinvoice_data::
		{
			Contact, EmployeeStatus,
			views::{ContactView, LocationView},
		},
		std::{collections::HashSet, time::Instant},
	};

	#[test]
	fn test_into_view()
	{
		let start = Instant::now();

		util::test_temp_store(|store|
		{
			let earth = BincodeLocation::create("Earth", *store).unwrap();

			let big_old_test = BincodeOrganization::create(
				earth.location.clone(),
				"Big Old Test Corporation",
				*store,
			).unwrap();

			let mut contact_info = HashSet::new();
			contact_info.insert(Contact::Address(earth.location.id));

			let testy = BincodePerson::create(
				contact_info.clone(),
				"Testy Mćtesterson",
				*store,
			).unwrap();

			let ceo_testy = BincodeEmployee::create(
				contact_info.clone(),
				big_old_test.organization.clone(),
				testy.person.clone(),
				"CEO of Tests",
				EmployeeStatus::Employed,
				*store,
			).unwrap();

			let earth_view = LocationView
			{
				name: earth.location.name,
				outer: None,
			};

			let contact_info_view: HashSet<ContactView> = [
				ContactView::Address(earth_view.clone())
			].iter().cloned().collect();

			let ceo_testy_view = EmployeeView
			{
				contact_info: contact_info_view.clone(),
				organization: OrganizationView
				{
					location: earth_view,
					name: big_old_test.organization.name,
				},
				person: PersonView
				{
					contact_info: contact_info_view,
					id: testy.person.id,
					name: testy.person.name,
				},
				title: ceo_testy.employee.title.clone(),
				status: ceo_testy.employee.status,
			};

			let ceo_testy_view_result: DynamicResult<EmployeeView> = ceo_testy.into();

			// Asser that the synthetic view is the same as the view which was created naturally.
			assert_eq!(ceo_testy_view, ceo_testy_view_result.unwrap());

			println!("\n>>>>> BincodeEmployee test_into_view {}us <<<<<\n", Instant::now().duration_since(start).as_micros());
		});
	}
}
