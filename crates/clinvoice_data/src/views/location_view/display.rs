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
	use std::time::Instant;

	use super::LocationView;
	#[cfg(uuid)]
	use crate::Id;

	/// # Summary
	///
	/// The main method.
	#[test]
	fn display()
	{
		let earth_view = LocationView {
			name: "Earth".into(),
			#[cfg(uuid)]
			id: Id::new_v4(),
			#[cfg(not(uuid))]
			id: 0,
			outer: None,
		};

		let usa_view = LocationView {
			name: "USA".into(),
			#[cfg(uuid)]
			id: Id::new_v4(),
			#[cfg(not(uuid))]
			id: 0,
			outer: Some(earth_view.into()),
		};

		let arizona_view = LocationView {
			name: "Arizona".into(),
			#[cfg(uuid)]
			id: Id::new_v4(),
			#[cfg(not(uuid))]
			id: 0,
			outer: Some(usa_view.into()),
		};

		let phoenix_view = LocationView {
			name: "Phoenix".into(),
			#[cfg(uuid)]
			id: Id::new_v4(),
			#[cfg(not(uuid))]
			id: 0,
			outer: Some(arizona_view.into()),
		};

		let street_view = LocationView {
			name: "1337 Some Street".into(),
			#[cfg(uuid)]
			id: Id::new_v4(),
			#[cfg(not(uuid))]
			id: 0,
			outer: Some(phoenix_view.into()),
		};

		let start = Instant::now();
		assert_eq!(
			format!("{}", street_view),
			"1337 Some Street, Phoenix, Arizona, USA, Earth"
		);
		println!(
			"\n>>>>> LocationView::fmt {}us <<<<<\n",
			Instant::now().duration_since(start).as_micros()
		);
	}
}
