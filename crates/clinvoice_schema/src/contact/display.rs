use core::fmt::{Display, Formatter, Result};

use super::Contact;

impl Display for Contact
{
	fn fmt(&self, formatter: &mut Formatter) -> Result
	{
		match self
		{
			Contact::Address {
				location,
				export: _,
			} => location.fmt(formatter),
			Contact::Email {
				email: s,
				export: _,
			} |
			Contact::Phone {
				phone: s,
				export: _,
			} => write!(formatter, "{s}"),
		}
	}
}

#[cfg(test)]
mod tests
{
	use super::Contact;
	use crate::Location;

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
			format!("{}", Contact::Address {
				location: street_view,
				export: false,
			}),
			"1337 Some Street, Phoenix, Arizona, USA, Earth"
		);
		assert_eq!(
			format!("{}", Contact::Email {
				email: "foo@bar.io".into(),
				export: false,
			}),
			"foo@bar.io"
		);
		assert_eq!(
			format!("{}", Contact::Phone {
				phone: "1-603-555-5555".into(),
				export: false,
			}),
			"1-603-555-5555"
		);
	}
}
