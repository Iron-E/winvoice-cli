use core::fmt::{Display, Formatter, Result};

use super::PersonView;

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
	use std::time::Instant;

	use super::PersonView;

	#[test]
	fn display()
	{
		let person_view = PersonView {
			id:   0,
			name: "Someone".into(),
		};

		let start = Instant::now();
		assert_eq!(
			format!("{}", person_view),
			format!("#{}: Someone", person_view.id)
		);
		println!(
			"\n>>>>> PersonView::fmt {}us <<<<<\n",
			Instant::now().duration_since(start).as_micros()
		);
	}
}
