use clap::Subcommand as Clap;

use super::{create::Create, delete::Delete, init::Init, retrieve::Retrieve, update::Update};

/// The specific command that CLInvoice should run.
#[derive(Clap, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Command
{
	/// Edit the CLInvoice configuration file in your default editor.
	///
	/// Setting your default editor depends on platform. On Unix-based systems, try setting
	/// `$EDITOR`.
	Config,

	#[allow(missing_docs)]
	Create(Create),

	#[allow(missing_docs)]
	Delete(Delete),

	#[allow(missing_docs)]
	Init(Init),

	#[allow(missing_docs)]
	Retrieve(Retrieve),

	#[allow(missing_docs)]
	Update(Update),
}
