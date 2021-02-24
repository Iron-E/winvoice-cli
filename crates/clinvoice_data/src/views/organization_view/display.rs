use
{
	super::OrganizationView,
	std::fmt::{Display, Formatter, Result},
};

impl Display for OrganizationView
{
	fn fmt(&self, formatter: &mut Formatter) -> Result
	{
		return write!(formatter, "{} @ {}", self.name, self.location);
	}
}

#[cfg(test)]
mod tests
{
	use
	{
		super::OrganizationView,
		crate::{Id, views::LocationView},
		std::time::Instant,
	};

	#[test]
	fn test_display()
	{
		let organization = OrganizationView
		{
			id: Id::new_v4(),
			location: LocationView
			{
				id: Id::new_v4(),
				name: "Arizona".into(),
				outer: Some(LocationView
				{
					id: Id::new_v4(),
					name: "USA".into(),
					outer: Some(LocationView
					{
						id: Id::new_v4(),
						name: "Earth".into(),
						outer: None,
					}.into()),
				}.into()),
			},
			name: "Big Old Test".into(),
		};

		let start = Instant::now();
		assert_eq!(format!("{}", organization), "Big Old Test @ Arizona, USA, Earth");
		println!("\n>>>>> OrganizationView::fmt {}us <<<<<\n", Instant::now().duration_since(start).as_micros());
	}
}
