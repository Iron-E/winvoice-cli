use core::fmt::{Display, Formatter, Result};

use super::Location;

impl Display for Location
{
	fn fmt(&self, formatter: &mut Formatter) -> Result
	{
		write!(formatter, "{}", self.name)?;

		let mut outer = &self.outer;
		while let Some(o) = outer
		{
			write!(formatter, ", {}", o.name)?;
			outer = &o.outer;
		}

		Ok(())
	}
}

#[cfg(test)]
mod tests
{
	use pretty_assertions::assert_eq;

	use super::Location;

	#[test]
	fn display()
	{
		let earth_view = Location {
			id: 0,
			name: "Earth".into(),
			outer: None,
		};

		let usa_view = Location {
			id: 0,
			name: "USA".into(),
			outer: Some(earth_view.into()),
		};

		let arizona_view = Location {
			id: 0,
			name: "Arizona".into(),
			outer: Some(usa_view.into()),
		};

		let phoenix_view = Location {
			id: 0,
			name: "Phoenix".into(),
			outer: Some(arizona_view.into()),
		};

		let street_view = Location {
			id: 0,
			name: "1337 Some Street".into(),
			outer: Some(phoenix_view.into()),
		};

		assert_eq!(
			street_view.to_string(),
			"1337 Some Street, Phoenix, Arizona, USA, Earth"
		);
	}
}
