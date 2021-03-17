use
{
	super::EmployeeView,
	std::fmt::{Display, Formatter, Result},
};

impl Display for EmployeeView
{
	fn fmt(&self, formatter: &mut Formatter) -> Result
	{
		writeln!(formatter, "{} {}", self.title, self.person.name)?;
		writeln!(formatter, "\tEmployer: {}", self.organization)?;
		writeln!(formatter, "\tEmployee Contact Info:")?;
		{
			let mut sorted_employee_contact_info: Vec<String> = self.contact_info.keys().cloned().collect();
			sorted_employee_contact_info.sort();
			sorted_employee_contact_info.iter().try_for_each(|c| writeln!(formatter, "\t\t- {}: {}", c, self.contact_info[c]))?;
		}

		writeln!(formatter, "\tPersonal Contact Info:")?;
		{
			let mut sorted_person_contact_info: Vec<String> = self.person.contact_info.keys().cloned().collect();
			sorted_person_contact_info.sort();
			sorted_person_contact_info.iter().try_for_each(|c| writeln!(formatter, "\t\t- {}: {}", c, self.person.contact_info[c]))?;
		}

		write!(formatter, "\tStatus: {}", self.status)
	}
}

#[cfg(test)]
mod tests
{
	use
	{
		super::EmployeeView,
		crate::
		{
			Id, EmployeeStatus,
			views::{ContactView, LocationView, OrganizationView, PersonView},
		},
		std::time::Instant,
	};

	#[test]
	fn test_display()
	{
		let earth_view = LocationView
		{
			name: "Earth".into(),
			id: Id::new_v4(),
			outer: None,
		};

		let usa_view = LocationView
		{
			name: "USA".into(),
			id: Id::new_v4(),
			outer: Some(earth_view.into()),
		};

		let arizona_view = LocationView
		{
			name: "Arizona".into(),
			id: Id::new_v4(),
			outer: Some(usa_view.into())
		};

		let phoenix_view = LocationView
		{
			name: "Phoenix".into(),
			id: Id::new_v4(),
			outer: Some(arizona_view.into()),
		};

		let home_street_view = LocationView
		{
			name: "1337 Home Road".into(),
			id: Id::new_v4(),
			outer: Some(phoenix_view.clone().into()),
		};

		let work_street_view = LocationView
		{
			name: "1234 Work Street".into(),
			id: Id::new_v4(),
			outer: Some(phoenix_view.into()),
		};

		let employee = EmployeeView
		{
			contact_info: vec![
				("Place of Work".into(), work_street_view.clone().into()),
				("Work Email".into(), ContactView::Email("foo@bar.io".into())),
			].into_iter().collect(),
			id: Id::new_v4(),
			organization: OrganizationView
			{
				id: Id::new_v4(),
				location: work_street_view,
				name: "Big Old Test".into(),
			},
			person: PersonView
			{
				contact_info: vec![
					("Home".into(), home_street_view.clone().into()),
					("Personal Email".into(), ContactView::Email("bar@foo.io".into())),
				].into_iter().collect(),
				id: Id::new_v4(),
				name: "Testy McTesterson".into(),
			},
			status: EmployeeStatus::Representative,
			title: "CEO of Tests".into(),
		};

		let start = Instant::now();
		assert_eq!(
			format!("{}", employee),
"CEO of Tests Testy McTesterson
	Employer: Big Old Test @ 1234 Work Street, Phoenix, Arizona, USA, Earth
	Employee Contact Info:
		- Place of Work: 1234 Work Street, Phoenix, Arizona, USA, Earth
		- Work Email: foo@bar.io
	Personal Contact Info:
		- Home: 1337 Home Road, Phoenix, Arizona, USA, Earth
		- Personal Email: bar@foo.io
	Status: Representative",
		);
		println!("\n>>>>> EmployeeView::fmt {}us <<<<<\n", Instant::now().duration_since(start).as_micros());
	}
}
