use core::fmt::{Display, Formatter, Result};

use super::Contact;

impl Display for Contact
{
	fn fmt(&self, formatter: &mut Formatter) -> Result
	{
		write!(formatter, "{}: {}", self.label, self.kind)
	}
}

#[cfg(test)]
mod tests
{
	use super::Contact;
	use crate::{ContactKind, Location};

	/// # Summary
	///
	/// The main method.
	#[test]
	fn display()
	{
		let earth_view = Location {
			name: "Earth".into(),
			id: 0,
			outer: None,
		};

		let usa_view = Location {
			name: "USA".into(),
			id: 0,
			outer: Some(earth_view.into()),
		};

		let arizona_view = Location {
			name: "Arizona".into(),
			id: 0,
			outer: Some(usa_view.into()),
		};

		let phoenix_view = Location {
			name: "Phoenix".into(),
			id: 0,
			outer: Some(arizona_view.into()),
		};

		let street_view = Location {
			name: "1337 Some Street".into(),
			id: 0,
			outer: Some(phoenix_view.into()),
		};

		assert_eq!(
			format!("{}", Contact {
				export: false,
				kind: ContactKind::Address(street_view),
				label: "Office".into(),
				organization_id: Default::default(),
			}),
			"Office: 1337 Some Street, Phoenix, Arizona, USA, Earth"
		);
		assert_eq!(
			format!("{}", Contact {
				export: false,
				kind: ContactKind::Email("foo@bar.io".into()),
				label: "Email".into(),
				organization_id: Default::default(),
			}),
			"Email: foo@bar.io"
		);
		assert_eq!(
			format!("{}", Contact {
				export: false,
				kind: ContactKind::Phone("1-603-555-5555".into()),
				label: "Cellphone".into(),
				organization_id: Default::default(),
			}),
			"Cellphone: 1-603-555-5555"
		);
	}
}
