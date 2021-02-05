use
{
	super::BincodeEmployee,
	crate::data::{BincodeJob, BincodeOrganization},
	clinvoice_adapter::data::{Deletable, JobAdapter, MatchWhen, OrganizationAdapter},
	std::{error::Error, fs, io::ErrorKind},
};

impl Deletable for BincodeEmployee<'_, '_, '_>
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
			for result in BincodeJob::retrieve(
				MatchWhen::Any, // client
				MatchWhen::Any, // date close
				MatchWhen::Any, // date open
				MatchWhen::Any, // id
				MatchWhen::Any, // invoice date issued
				MatchWhen::Any, // invoice date paid
				MatchWhen::Any, // invoice hourly rate
				MatchWhen::Any, // notes
				MatchWhen::Any, // objectives
				MatchWhen::HasAny([self.employee.id].iter().cloned().collect()), // timesheet employee
				MatchWhen::Any, // timesheet time begin
				MatchWhen::Any, // timesheet time end
				self.store,
			)? { result.delete(true)?; }

			for result in BincodeOrganization::retrieve(
				MatchWhen::Any, // id
				MatchWhen::Any, // location
				MatchWhen::Any, // name
				MatchWhen::HasAll([self.employee.id].iter().cloned().collect()), // representatives
				self.store,
			)? { result.delete(true)?; }
		}

		return Ok(());
	}
}

#[cfg(test)]
mod tests
{
	use crate::util;
	use std::time::Instant;

	#[test]
	fn test_delete()
	{
		let start = Instant::now();

		util::test_temp_store(|store|
		{
			// TODO

			println!("\n>>>>> BincodeEmployee test_delete {}us <<<<<\n", Instant::now().duration_since(start).as_micros());
		});
	}
}
