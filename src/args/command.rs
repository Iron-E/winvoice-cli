use clap::Subcommand as Clap;
use clinvoice_config::{Config, Store};

use super::{create::Create, delete::Delete, retrieve::Retrieve, update::Update};
use crate::DynResult;

/// The specific command that CLInvoice should run.
#[derive(Clap, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Command
{
	/// Edit the CLInvoice configuration file in your default editor.
	///
	/// Setting your default editor depends on platform. On Unix-based systems, try setting
	/// `$EDITOR`.
	Config,

	#[clap(subcommand)]
	Create(Create),

	Delete(Delete),

	/// Prepare the specified store (-s) for use with CLInvoice.
	///
	/// Will not clobber existing data. Should only be run by administrators.
	Init,

	Retrieve(Retrieve),

	Update(Update),
}
