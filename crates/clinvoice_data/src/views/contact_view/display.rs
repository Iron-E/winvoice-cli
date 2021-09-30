use core::fmt::{Display, Formatter, Result};

use super::ContactView;

impl Display for ContactView
{
	fn fmt(&self, formatter: &mut Formatter) -> Result
	{
		match self
		{
			ContactView::Address {
				location,
				export: _,
			} => location.fmt(formatter),
			ContactView::Email { email, export: _ } => write!(formatter, "{}", email),
			ContactView::Phone { phone, export: _ } => write!(formatter, "{}", phone),
		}
	}
}

#[cfg(test)]
mod tests
{
	use std::time::Instant;

	use super::ContactView;
	use crate::views::LocationView;

	/// # Summary
	///
	/// The main method.
	#[test]
	fn display()
	{
		let earth_view = LocationView {
			name: "Earth".into(),
			id: 0,
			outer: None,
		};

		let usa_view = LocationView {
			name: "USA".into(),
			id: 0,
			outer: Some(earth_view.into()),
		};

		let arizona_view = LocationView {
			name: "Arizona".into(),
			id: 0,
			outer: Some(usa_view.into()),
		};

		let phoenix_view = LocationView {
			name: "Phoenix".into(),
			id: 0,
			outer: Some(arizona_view.into()),
		};

		let street_view = LocationView {
			name: "1337 Some Street".into(),
			id: 0,
			outer: Some(phoenix_view.into()),
		};

		let start = Instant::now();
		assert_eq!(
			format!("{}", ContactView::Address {
				location: street_view,
				export:   false,
			}),
			"1337 Some Street, Phoenix, Arizona, USA, Earth"
		);
		assert_eq!(
			format!("{}", ContactView::Email {
				email:  "foo@bar.io".into(),
				export: false,
			}),
			"foo@bar.io"
		);
		assert_eq!(
			format!("{}", ContactView::Phone {
				phone:  "1-603-555-5555".into(),
				export: false,
			}),
			"1-603-555-5555"
		);
		println!(
			"\n>>>>> ContactView::test_display {}us <<<<<\n",
			Instant::now().duration_since(start).as_micros() / 3
		);
	}
}
