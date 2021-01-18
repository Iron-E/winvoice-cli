use super::TomlEmployee;
use clinvoice_data::Organization;
use std::error::Error;

impl<'err, 'name> Into<Result<Organization<'name>, &'err dyn Error>> for TomlEmployee<'_, '_, '_, '_, '_, '_, '_>
{
	fn into(self) -> Result<Organization<'name>, &'err dyn Error>
	{
		todo!()
	}
}
