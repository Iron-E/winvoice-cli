use core::fmt::{Display, Formatter, Result};

use super::EmployeeView;

impl Display for EmployeeView
{
	fn fmt(&self, formatter: &mut Formatter) -> Result
	{
		writeln!(formatter, "{} {}", self.title, self.person.name)?;
		writeln!(formatter, "\tEmployer: {}", self.organization)?;

		if !self.contact_info.is_empty()
		{
			writeln!(formatter, "\tEmployee Contact Info:")?;

			let mut sorted_employee_contact_info: Vec<&String> = self.contact_info.keys().collect();
			sorted_employee_contact_info.sort();
			sorted_employee_contact_info
				.into_iter()
				.try_for_each(|c| writeln!(formatter, "\t\t- {}: {}", c, self.contact_info[c]))?;
		}

		write!(formatter, "\tStatus: {}", self.status)
	}
}

#[cfg(test)]
mod tests
{
	use super::EmployeeView;
	use crate::{
		views::{ContactView, LocationView, OrganizationView, PersonView},
		EmployeeStatus,
	};

	#[test]
	fn display()
	{
		let earth_view = LocationView {
			id: 0,
			name: "Earth".into(),
			outer: None,
		};

		let usa_view = LocationView {
			id: 0,
			name: "USA".into(),
			outer: Some(earth_view.into()),
		};

		let arizona_view = LocationView {
			id: 0,
			name: "Arizona".into(),
			outer: Some(usa_view.into()),
		};

		let phoenix_view = LocationView {
			id: 0,
			name: "Phoenix".into(),
			outer: Some(arizona_view.into()),
		};

		let work_street_view = LocationView {
			id: 0,
			name: "1234 Work Street".into(),
			outer: Some(phoenix_view.into()),
		};

		let employee = EmployeeView {
			contact_info: vec![
				("Place of Work".into(), ContactView::Address {
					location: work_street_view.clone(),
					export: false,
				}),
				("Work Email".into(), ContactView::Email {
					email: "foo@bar.io".into(),
					export: false,
				}),
			]
			.into_iter()
			.collect(),
			id: 0,
			organization: OrganizationView {
				id: 0,
				location: work_street_view,
				name: "Big Old Test".into(),
			},
			person: PersonView {
				id: 0,
				name: "Testy McTesterson".into(),
			},
			status: EmployeeStatus::Representative,
			title: "CEO of Tests".into(),
		};

		assert_eq!(
			format!("{}", employee),
			"CEO of Tests Testy McTesterson
	Employer: Big Old Test @ 1234 Work Street, Phoenix, Arizona, USA, Earth
	Employee Contact Info:
		- Place of Work: 1234 Work Street, Phoenix, Arizona, USA, Earth
		- Work Email: foo@bar.io
	Status: Representative",
		);
	}
}
