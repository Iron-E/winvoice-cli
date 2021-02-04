use
{
	super::BincodeOrganization,
	crate::data::{BincodeEmployee, BincodeJob},
	clinvoice_adapter::data::{Deletable, EmployeeAdapter, JobAdapter, MatchWhen},
	std::{error::Error, fs, io::ErrorKind},
};

impl Deletable for BincodeOrganization<'_, '_, '_>
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
				MatchWhen::EqualTo(self.organization.id), // client
				MatchWhen::Any, // date close
				MatchWhen::Any, // date open
				MatchWhen::Any, // id
				MatchWhen::Any, // invoice date issued
				MatchWhen::Any, // invoice date paid
				MatchWhen::Any, // invoice hourly rate
				MatchWhen::Any, // notes
				MatchWhen::Any, // objectives
				MatchWhen::Any, // timesheet employee
				MatchWhen::Any, // timesheet time begin
				MatchWhen::Any, // timesheet time end
				self.store,
			)? { result.delete(true)?; }

			for result in BincodeEmployee::retrieve(
				MatchWhen::Any, // contact info
				MatchWhen::Any, // employed
				MatchWhen::Any, // id
				MatchWhen::EqualTo(self.organization.id), // organization
				MatchWhen::Any, // person
				MatchWhen::Any, // title
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

			println!("\n>>>>> BincodeOrganiztaion test_delete {}us <<<<<\n", Instant::now().duration_since(start).as_micros());
		}).unwrap();
	}
}
