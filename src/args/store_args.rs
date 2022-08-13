#[cfg(test)]
mod from;

use clap::Args as Clap;
use clinvoice_config::{Config, Error, Result, Store};

/// Reusable arguments used for specifying a store.
#[derive(Clap, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct StoreArgs
{
	/// A key from the `[stores]` section of the [configuration file](clinvoice_config::Config).
	#[clap(
		default_value = "default",
		help = "A key from the `[stores]` section of the configuration file",
		long,
		short
	)]
	store: String,
}

impl StoreArgs
{
	/// Try to get the store named `store_name` from `config` and return it, erroring if it does not
	/// exist.
	pub fn try_get_from<'connection>(
		&self,
		config: &'connection Config,
	) -> Result<&'connection Store>
	{
		config
			.get_store(&self.store)
			.ok_or_else(|| Error::NotConfigured(self.store.clone(), "stores".into()))
	}
}
