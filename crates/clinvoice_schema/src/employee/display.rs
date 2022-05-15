use core::fmt::{Display, Formatter, Result};

use super::Employee;

impl Display for Employee
{
	fn fmt(&self, formatter: &mut Formatter) -> Result
	{
		writeln!(formatter, "{} {}", self.title, self.person.name)?;
		writeln!(formatter, "\tEmployer: {}", self.organization)?;

		if !self.contact_info.is_empty()
		{
			writeln!(formatter, "\tEmployee Contact Info:")?;

			let mut sorted_employee_contact_info = self.contact_info.clone();
			sorted_employee_contact_info.sort_by(|c1, c2| c1.label.cmp(&c2.label));
			sorted_employee_contact_info
				.into_iter()
				.try_for_each(|c| writeln!(formatter, "\t\t- {c}"))?;
		}

		write!(formatter, "\tStatus: {}", self.status)
	}
}

#[cfg(test)]
mod tests
{
	use super::Employee;
	use crate::{Contact, ContactKind, Location, Organization, Person};

	#[test]
	fn display()
	{
		let earth_view = Location {
			id: 0,
			name: "Earth".into(),
			outer: None,
		};

		let usa_view = Location {
			id: 0,
			name: "USA".into(),
			outer: Some(earth_view.into()),
		};

		let arizona_view = Location {
			id: 0,
			name: "Arizona".into(),
			outer: Some(usa_view.into()),
		};

		let phoenix_view = Location {
			id: 0,
			name: "Phoenix".into(),
			outer: Some(arizona_view.into()),
		};

		let work_street_view = Location {
			id: 0,
			name: "1234 Work Street".into(),
			outer: Some(phoenix_view.into()),
		};

		let employee = Employee {
			contact_info: vec![
				Contact {
					employee_id: 0,
					kind: ContactKind::Address(work_street_view.clone()),
					label: "Place of Work".into(),
					export: false,
				},
				Contact {
					employee_id: 0,
					kind: ContactKind::Email("foo@bar.io".into()),
					label: "Work Email".into(),
					export: false,
				},
			],
			id: 0,
			organization: Organization {
				id: 0,
				location: work_street_view,
				name: "Big Old Test".into(),
			},
			person: Person {
				id: 0,
				name: "Testy McTesterson".into(),
			},
			status: "Representative".into(),
			title: "CEO of Tests".into(),
		};

		assert_eq!(
			format!("{employee}"),
			"CEO of Tests Testy McTesterson
	Employer: Big Old Test @ 1234 Work Street, Phoenix, Arizona, USA, Earth
	Employee Contact Info:
		- Place of Work: 1234 Work Street, Phoenix, Arizona, USA, Earth
		- Work Email: foo@bar.io
	Status: Representative",
		);
	}
}
