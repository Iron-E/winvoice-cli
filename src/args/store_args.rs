use clap::Args as Clap;
use clinvoice_config::{Config, Store};

/// Reusable arguments used for specifying a store.
#[derive(Clap, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct StoreArgs
{
	/// A key from the `[stores]` section of the [configuration file](clinvoice_config::Config).
	#[clap(
		default_value = "default",
		help = "A key from the `[stores]` section of the configuration file.",
		long,
		short
	)]
	store: String,
}

impl StoreArgs
{
	/// Try to get the store named `store_name` from `config` and return it, erroring if it does not
	/// exist.
	pub fn try_get_from<'c>(&self, config: &'c Config) -> Result<&'c Store, String>
	{
		config.get_store(&self.store).ok_or_else(|| {
			format!(
				r#"The store named "{}" was not found in your configuration file."#,
				self.store,
			)
		})
	}
}
