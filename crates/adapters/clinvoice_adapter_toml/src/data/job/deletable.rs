use super::TomlJob;
use clinvoice_adapter::{data::Deletable, Store};
use clinvoice_data::{chrono::TimeZone, Id};
use std::error::Error;

impl<'pass, 'path, 'user, TZone> Deletable<'pass, 'path, 'user>
for TomlJob<'_, '_, '_, '_, 'pass, 'path, 'user, TZone>
where TZone : TimeZone
{
	fn delete(store: Store<'pass, 'path, 'user>, id: Id, cascade: bool) -> Result<(), Box<dyn Error>>
	{
		todo!()
	}
}
