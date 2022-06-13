use serde::{Deserialize, Serialize};

use crate::Adapters;

/// # Summary
///
/// A place for CLInvoice to store information.
///
/// # Remarks
///
/// If this application is being used by an organization, this configuration should be setup by an
/// administrator.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Store
{
	/// # Summary
	///
	/// The adapter to use for this [`Store`].
	pub adapter: Adapters,

	/// # Summary
	///
	/// The place where the data can be found.
	///
	/// # Remarks
	///
	/// The specifics of how this option is formed depends on the `adapter`.
	/// [The docs](https://github.com/Iron-E/clinvoice/wiki/Usage#adapters) for more information.
	pub url: String,
}
