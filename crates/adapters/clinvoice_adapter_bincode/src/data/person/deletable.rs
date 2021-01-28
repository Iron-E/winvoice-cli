use super::BincodePerson;
use clinvoice_adapter::{data::Deletable, Store};
use clinvoice_data::Id;
use std::error::Error;

impl<'pass, 'path, 'user> Deletable<'pass, 'path, 'user> for BincodePerson<'_, '_, '_, 'pass, 'path, 'user>
{
	fn delete(&self, cascade: bool) -> Result<(), Box<dyn Error>>
	{
		todo!()
	}
}
