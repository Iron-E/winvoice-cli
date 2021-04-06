use
{
	core::fmt::{Display, Formatter, Result},

	super::ContactView,
};

impl Display for ContactView
{
	fn fmt(&self, formatter: &mut Formatter) -> Result
	{
		match self
		{
			ContactView::Address(location) => location.fmt(formatter),
			ContactView::Email(email) => write!(formatter, "{}", email),
			ContactView::Phone(phone) => write!(formatter, "{}", phone),
		}
	}
}

#[cfg(test)]
mod tests
{
	use
	{
		std::time::Instant,

		super::ContactView,
		crate::{Id, views::LocationView},
	};

	/// # Summary
	///
	/// The main method.
	#[test]
	fn display()
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

		let street_view = LocationView
		{
			name: "1337 Some Street".into(),
			id: Id::new_v4(),
			outer: Some(phoenix_view.into()),
		};

		let start = Instant::now();
		assert_eq!(format!("{}", ContactView::from(street_view)), "1337 Some Street, Phoenix, Arizona, USA, Earth");
		assert_eq!(format!("{}", ContactView::Email("foo@bar.io".into())), "foo@bar.io");
		assert_eq!(format!("{}", ContactView::Phone("1-603-555-5555".into())), "1-603-555-5555");
		println!("\n>>>>> ContactView::test_display {}us <<<<<\n", Instant::now().duration_since(start).as_micros() / 3);
	}
}
