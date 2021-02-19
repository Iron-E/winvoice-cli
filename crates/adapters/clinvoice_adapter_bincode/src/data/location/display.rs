use
{
	super::BincodeLocation,
	clinvoice_adapter::data::LocationAdapter,
	core::fmt::{Display, Formatter, Error, Result as FmtResult},
};

impl Display for BincodeLocation<'_, '_, '_>
{
	/// # Summary
	///
	/// Format some given [`Location`] so that all of its [containing outer
	/// `Location`](Location::outer_id)s come before it.
	fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult
	{
		let outer_locations = match self.outer_locations()
		{
			Ok(locations) => locations,
			_ => Err(Error)?,
		};

		return write!(formatter, "{}", outer_locations.iter().fold(
			String::from(&self.location.name),
			|out, loc| out + ", " + &loc.name
		));
	}
}

#[cfg(test)]
mod tests
{
	use
	{
		super::{BincodeLocation, LocationAdapter},
		crate::util,
		std::time::Instant,
	};

	#[test]
	fn test_display()
	{
		let start = Instant::now();

		util::test_temp_store(|store|
		{
			let earth = BincodeLocation::create("Earth", *store).unwrap();
			let usa = earth.create_inner("USA").unwrap();
			let arizona = usa.create_inner("Arizona").unwrap();
			let phoenix = arizona.create_inner("Phoenix").unwrap();

			assert_eq!(phoenix.to_string(), "Phoenix, Arizona, USA, Earth");

			println!("\n>>>>> BincodeLocation test_display {}us <<<<<\n", Instant::now().duration_since(start).as_micros());
		});
	}
}
