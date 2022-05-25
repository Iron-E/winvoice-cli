use core::fmt::{Display, Formatter, Result};

use super::Employee;

impl Display for Employee
{
	fn fmt(&self, formatter: &mut Formatter) -> Result
	{
		writeln!(formatter, "{} {}", self.title, self.name)?;

		const DEPTH_2: &str = "\n\t\t";
		writeln!(
			formatter,
			"\tEmployer: {}",
			self.organization.to_string().replace('\n', DEPTH_2)
		)?;

		write!(formatter, "\tStatus: {}", self.status)
	}
}

#[cfg(test)]
mod tests
{
	use super::Employee;
	use crate::{Contact, ContactKind, Location, Organization};

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
			id: 0,
			organization: Organization {
				contact_info: vec![
					Contact {
						export: false,
						kind: ContactKind::Address(work_street_view.clone()),
						label: "Place of Work".into(),
						organization_id: 0,
					},
					Contact {
						export: false,
						kind: ContactKind::Email("foo@bar.io".into()),
						label: "Work Email".into(),
						organization_id: 0,
					},
				],
				id: 0,
				location: work_street_view,
				name: "Big Old Test".into(),
			},
			name: "Testy McTesterson".into(),
			status: "Representative".into(),
			title: "CEO of Tests".into(),
		};

		assert_eq!(
			format!("{employee}"),
			"CEO of Tests Testy McTesterson
	Employer: Big Old Test @ 1234 Work Street, Phoenix, Arizona, USA, Earth
		Contact Info:
			- Place of Work: 1234 Work Street, Phoenix, Arizona, USA, Earth
			- Work Email: foo@bar.io
	Status: Representative",
		);
	}
}
