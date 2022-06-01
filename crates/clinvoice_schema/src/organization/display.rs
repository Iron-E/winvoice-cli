use core::fmt::{Display, Formatter, Result};

use super::Organization;

impl Display for Organization
{
	fn fmt(&self, formatter: &mut Formatter) -> Result
	{
		write!(formatter, "{} @ {}", self.name, self.location)?;

		if !self.contact_info.is_empty()
		{
			write!(formatter, "\n\t- Contact Info:")?;

			/// # Summary
			///
			/// Two indents in, with a newline.
			const DEPTH_2: &str = "\n\t\t";

			let mut sorted_employee_contact_info = self.contact_info.clone();
			sorted_employee_contact_info.sort_by(|c1, c2| c1.label.cmp(&c2.label));
			sorted_employee_contact_info
				.into_iter()
				.try_for_each(|c| write!(formatter, "{DEPTH_2}- {c}"))?;
		}

		Ok(())
	}
}

#[cfg(test)]
mod tests
{
	use super::Organization;
	use crate::{Contact, ContactKind, Location};

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

		let organization = Organization {
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
			location: Location {
				id: 0,
				name: "Arizona".into(),
				outer: Some(
					Location {
						id: 0,
						name: "USA".into(),
						outer: Some(
							Location {
								id: 0,
								name: "Earth".into(),
								outer: None,
							}
							.into(),
						),
					}
					.into(),
				),
			},
			name: "Big Old Test".into(),
		};

		assert_eq!(
			format!("{organization}"),
			"Big Old Test @ Arizona, USA, Earth
	- Contact Info:
		- Place of Work: 1234 Work Street, Phoenix, Arizona, USA, Earth
		- Work Email: foo@bar.io"
		);
	}
}
