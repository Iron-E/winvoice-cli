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
	use super::Location;

	#[test]
	fn display()
	{
		let earth_view = Location {
			name: "Earth".into(),
			id: 0,
			outer: None,
		};

		let usa_view = Location {
			name: "USA".into(),
			id: 0,
			outer: Some(earth_view.into()),
		};

		let arizona_view = Location {
			name: "Arizona".into(),
			id: 0,
			outer: Some(usa_view.into()),
		};

		let phoenix_view = Location {
			name: "Phoenix".into(),
			id: 0,
			outer: Some(arizona_view.into()),
		};

		let street_view = Location {
			name: "1337 Some Street".into(),
			id: 0,
			outer: Some(phoenix_view.into()),
		};

		assert_eq!(
			format!("{street_view}"),
			"1337 Some Street, Phoenix, Arizona, USA, Earth"
		);
	}
}
