use
{
	super::BincodePerson,
	crate::data::BincodeEmployee,
	clinvoice_adapter::data::{Deletable, EmployeeAdapter, MatchWhen},
	std::{error::Error, fs, io::ErrorKind},
};

impl Deletable for BincodePerson<'_, '_, '_>
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
			for result in BincodeEmployee::retrieve(
				MatchWhen::Any,
				MatchWhen::Any,
				MatchWhen::Any,
				MatchWhen::Any,
				MatchWhen::EqualTo(self.person.id),
				self.store,
				MatchWhen::Any,
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
		super::{BincodePerson, Deletable, EmployeeAdapter},
		crate::{data::BincodeEmployee, util},
		clinvoice_adapter::data::PersonAdapter,
		std::{collections::HashSet, time::Instant},
	};

	#[test]
	fn test_delete()
	{
		let start = Instant::now();

		util::test_temp_store(|store|
		{
			// TODO

			println!("\n>>>>> BincodePerson test_delete {}us <<<<<\n", Instant::now().duration_since(start).as_micros());
		}).unwrap();
	}
}
