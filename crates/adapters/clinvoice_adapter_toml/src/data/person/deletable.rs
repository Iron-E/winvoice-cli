use super::TomlPerson;
use clinvoice_adapter::{data::Deletable, Store};
use clinvoice_data::Id;
use std::error::Error;

impl<'pass, 'path, 'user> Deletable<'pass, 'path, 'user>
for TomlPerson<'_, '_, '_, '_, 'pass, 'path, 'user>
{
	fn delete<'err>(store: Store<'pass, 'path, 'user>, id: Id, cascade: bool) -> Result<(), &'err dyn Error>
	{
		todo!()
	}
}
