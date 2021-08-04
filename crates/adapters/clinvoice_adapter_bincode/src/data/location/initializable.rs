use clinvoice_adapter::{
	data::Initializable,
	Store,
};

use super::BincodeLocation;
use crate::{
	data::{
		Error,
		Result,
	},
	util,
};

impl Initializable for BincodeLocation<'_, '_>
{
	type Error = Error;

	/// # Summary
	///
	/// Initialize the database for a given [`Store`].
	fn init(store: &Store) -> Result<()>
	{
		util::create_store_dir(&Self::path(store))?;
		Ok(())
	}
}
