use
{
	super::BincodeLocation,
	clinvoice_adapter::data::{LocationAdapter, MatchWhen},
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
		let mut output = String::from(&self.location.name);

		let mut outer_id = self.location.outer_id;
		while let Some(id) = outer_id
		{
			output += ", ";

			if let Ok(results) = BincodeLocation::retrieve(
				MatchWhen::EqualTo(id), // id
				MatchWhen::Any, // name
				MatchWhen::Any, // outer id
				self.store,
			)
			{
				if let Some(bincode_location) = results.iter().next()
				{
					output += &bincode_location.location.name;

					outer_id = bincode_location.location.outer_id;
					continue;
				}
			}

			return Err(Error);
		}

		write!(formatter, "{}", output)
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
		}).unwrap();
	}
}
