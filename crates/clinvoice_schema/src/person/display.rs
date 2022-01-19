use core::fmt::{Display, Formatter, Result};

use super::Person;

impl Display for Person
{
	fn fmt(&self, formatter: &mut Formatter) -> Result
	{
		write!(formatter, "#{}: {}", self.id, self.name)
	}
}

#[cfg(test)]
mod tests
{
	use super::Person;

	#[test]
	fn display()
	{
		let person_view = Person {
			id: 0,
			name: "Someone".into(),
		};

		assert_eq!(
			format!("{person_view}"),
			format!("#{}: Someone", person_view.id)
		);
	}
}
