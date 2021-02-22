use
{
	super::PersonView,
	std::fmt::{Display, Formatter, Result},
};

impl Display for PersonView
{
	fn fmt(&self, formatter: &mut Formatter<'_>) -> Result
	{
		let mut sorted_contact_info = self.contact_info.clone();
		sorted_contact_info.sort();

		writeln!(formatter, "Contact Info:")?;
		sorted_contact_info.iter().try_for_each(|c| writeln!(formatter, "\t- {}", c))?;

		return write!(formatter, "\nName: {}", self.name);
	}
}

#[cfg(test)]
mod tests
{
	use
	{
		super::PersonView,
		crate::
		{
			Id,
			views::{ContactView, LocationView}
		},
		std::time::Instant,
	};

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

		let contact_info = vec![
			ContactView::Address(street_view),
			ContactView::Email("foo@bar.io".into()),
			ContactView::Phone("1-800-555-5555".into()),
		];

		let person_view = PersonView
		{
			contact_info,
			id: Id::new_v4(),
			name: "Someone".into(),
		};

		assert_eq!(
			format!("{}", person_view),
			"Contact Info:\n\t- 1337 Some Street, Phoenix, Arizona, USA, Earth\n\t- foo@bar.io\n\t- 1-800-555-5555\n\nName: Someone"
		);

		println!("\n>>>>> PersonView test_display {}us <<<<<\n", Instant::now().duration_since(start).as_micros());
	}
}
