use super::TomlOrganization;
use clinvoice_data::Location;
use std::error::Error;

impl<'err, 'name> Into<Result<Location<'name>, &'err dyn Error>> for TomlOrganization<'name, '_, '_, '_>
{
	fn into(self) -> Result<Location<'name>, &'err dyn Error>
	{
		todo!()
	}
}

