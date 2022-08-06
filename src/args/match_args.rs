mod from;
mod try_into;

use std::path::PathBuf;

use clap::Args as Clap;

/// Reusable arguments used for retrieving information from a CLInvoice store.
#[derive(Clap, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MatchArgs
{
	/// A path to a YAML file that contains valid a match condition/query/search term for
	/// CLInvoice.
	///
	/// Because CLInvoice uses YAML files to search for items in `[stores]`, you can save the
	/// search terms and then reuse them any number of times. The file specified will be used
	/// for deserialized, and you will not be prompted to write a new search.
	#[clap(long, short, value_name = "FILE", value_parser)]
	r#match: Option<PathBuf>,
}
