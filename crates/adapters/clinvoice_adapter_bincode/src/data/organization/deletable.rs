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
				MatchWhen::EqualTo(self.organization.id),
				MatchWhen::Any,
				MatchWhen::Any,
				MatchWhen::Any,
				MatchWhen::Any,
				MatchWhen::Any,
				MatchWhen::Any,
				MatchWhen::Any,
				MatchWhen::Any,
				self.store,
			)? { result.delete(true)?; }

			for result in BincodeEmployee::retrieve(
				MatchWhen::Any,
				MatchWhen::Any,
				MatchWhen::Any,
				MatchWhen::EqualTo(self.organization.id),
				MatchWhen::Any,
				self.store,
				MatchWhen::Any,
			)? { result.delete(true)?; }
		}

		return Ok(());
	}
}
