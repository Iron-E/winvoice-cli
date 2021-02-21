use
{
	super::ContactView,
	std::fmt::{Display, Formatter, Result},
};

impl Display for ContactView
{
	fn fmt(&self, formatter: &mut Formatter<'_>) -> Result
	{
		return match self
		{
			ContactView::Address(location) => location.fmt(formatter),
			ContactView::Email(email) => writeln!(formatter, "{}", email),
			ContactView::Phone(phone) => writeln!(formatter, "{}", phone),
		};
	}
}

#[cfg(test)]
mod tests
{
	use
	{
		super::ContactView,
		crate::{Id, views::LocationView},
		std::time::Instant,
	};

	/// # Summary
	///
	/// The main method.
	#[test]
	fn test_display()
	{
		let start = Instant::now();

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

		assert_eq!(
			format!("{}", ContactView::Address(street_view)),
			"1337 Some Street, Phoenix, Arizona, USA, Earth"
		);

		assert_eq!(
			format!("{}", ContactView::Email("foo@bar.io".into())),
			"foo@bar.io"
		);

		assert_eq!(
			format!("{}", ContactView::Phone("1-603-555-5555".into())),
			"1-603-555-5555"
		);

		println!("\n>>>>> ContactView test_display {}us <<<<<\n", Instant::now().duration_since(start).as_micros());
	}
}
