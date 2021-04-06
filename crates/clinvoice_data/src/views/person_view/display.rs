use
{
	core::fmt::{Display, Formatter, Result},

	super::PersonView,
};

impl Display for PersonView
{
	fn fmt(&self, formatter: &mut Formatter) -> Result
	{
		write!(formatter, "#{}: {}", self.id, self.name)
	}
}

#[cfg(test)]
mod tests
{
	use
	{
		super::PersonView,
		crate::Id,
		std::time::Instant,
	};

	#[test]
	fn display()
	{
		let person_view = PersonView
		{
			id: Id::new_v4(),
			name: "Someone".into(),
		};

		let start = Instant::now();
		assert_eq!(format!("{}", person_view), format!("#{}: Someone", person_view.id));
		println!("\n>>>>> PersonView::fmt {}us <<<<<\n", Instant::now().duration_since(start).as_micros());
	}
}
