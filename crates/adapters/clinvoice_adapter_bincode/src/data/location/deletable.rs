use
{
	super::BincodeLocation,
	crate::data::BincodeOrganization,
	clinvoice_adapter::data::{Deletable, LocationAdapter, MatchWhen, OrganizationAdapter},
	std::{error::Error, fs, io::ErrorKind},
};

impl Deletable for BincodeLocation<'_, '_, '_>
{
	fn delete(&self, cascade: bool) -> Result<(), Box<dyn Error>>
	{
		if let Err(e) = fs::remove_file(self.filepath())
		{
			// We don't care if a file is missing; we want it deleted anyway.
			if e.kind() != ErrorKind::NotFound
			{
				return Err(e.into());
			}
		}

		if cascade
		{
			for result in BincodeLocation::retrieve(
				MatchWhen::Any, // id
				MatchWhen::Any, // name
				MatchWhen::EqualTo(Some(self.location.id)), // outer id
				self.store,
			)? { result.delete(true)?; }

			for result in BincodeOrganization::retrieve(
				MatchWhen::Any, // id
				MatchWhen::EqualTo(self.location.id), // location
				MatchWhen::Any, // name
				self.store,
			)? { result.delete(true)?; }
		}

		return Ok(());
	}
}

#[cfg(test)]
mod tests
{
	use
	{
		super::{BincodeLocation, Deletable, LocationAdapter},
		crate::{data::BincodeOrganization, util},
		clinvoice_adapter::data::OrganizationAdapter,
		std::time::Instant,
	};

	#[test]
	fn test_delete()
	{
		let start = Instant::now();

		util::test_temp_store(|store|
		{
			let earth = BincodeLocation::create("Earth", *store).unwrap();
			let usa = earth.create_inner("USA").unwrap();
			let arizona = usa.create_inner("Arizona").unwrap();
			let phoenix = arizona.create_inner("Phoenix").unwrap();

			let dogood = BincodeOrganization::create(
				arizona.location.clone(),
				"DoGood Inc",
				*store
			).unwrap();

			// delete just phoenix.
			phoenix.delete(false).unwrap();

			// assert that phoenix is gone.
			assert!(!&phoenix.filepath().is_file());

			// delete the usa and everything in it.
			usa.delete(true).unwrap();

			// Assert that every location inside the USA is gone
			assert!(&earth.filepath().is_file());
			assert!(!&usa.filepath().is_file());
			assert!(!&arizona.filepath().is_file());

			// assert that `dogood`, located in arizona, is gone.
			assert!(!&dogood.filepath().is_file());

			println!("\n>>>>> BincodeLocation test_delete {}us <<<<<\n", Instant::now().duration_since(start).as_micros());
		});
	}
}
