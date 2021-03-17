use
{
	super::PersonView,
	std::fmt::{Display, Formatter, Result},
};

impl Display for PersonView
{
	fn fmt(&self, formatter: &mut Formatter) -> Result
	{
		writeln!(formatter, "Name: {}", self.name)?;

		write!(formatter, "Contact Info:")?;
		{
			let mut sorted_employee_contact_info: Vec<String> = self.contact_info.keys().cloned().collect();
			sorted_employee_contact_info.sort();
			sorted_employee_contact_info.iter().try_for_each(|c| write!(formatter, "\n\t- {}: {}", c, self.contact_info[c]))?;
		}

		Ok(())
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
			views::{ContactView, LocationView},
		},
		std::time::Instant,
	};

	#[test]
	fn test_display()
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

		let contact_info = vec![
			("Home Address".into(), street_view.into()),
			("Personal Email".into(), ContactView::Email("foo@bar.io".into())),
			("Home Phone".into(), ContactView::Phone("1-800-555-5555".into())),
		].into_iter().collect();

		let person_view = PersonView
		{
			contact_info,
			id: Id::new_v4(),
			name: "Someone".into(),
		};

		let start = Instant::now();
		assert_eq!(
			format!("{}", person_view),
"Name: Someone
Contact Info:
	- Home Address: 1337 Some Street, Phoenix, Arizona, USA, Earth
	- Home Phone: 1-800-555-5555
	- Personal Email: foo@bar.io",
		);
		println!("\n>>>>> PersonView::fmt {}us <<<<<\n", Instant::now().duration_since(start).as_micros());
	}
}
