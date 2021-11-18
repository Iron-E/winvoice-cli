use core::fmt::{Display, Formatter, Result};

use super::LocationView;

impl Display for LocationView
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
	use super::LocationView;

	/// # Summary
	///
	/// The main method.
	#[test]
	fn display()
	{
		let earth_view = LocationView {
			name: "Earth".into(),
			id: 0,
			outer: None,
		};

		let usa_view = LocationView {
			name: "USA".into(),
			id: 0,
			outer: Some(earth_view.into()),
		};

		let arizona_view = LocationView {
			name: "Arizona".into(),
			id: 0,
			outer: Some(usa_view.into()),
		};

		let phoenix_view = LocationView {
			name: "Phoenix".into(),
			id: 0,
			outer: Some(arizona_view.into()),
		};

		let street_view = LocationView {
			name: "1337 Some Street".into(),
			id: 0,
			outer: Some(phoenix_view.into()),
		};

		assert_eq!(
			format!("{}", street_view),
			"1337 Some Street, Phoenix, Arizona, USA, Earth"
		);
	}
}
