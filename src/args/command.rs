use clap::Subcommand as Clap;

use super::{create::Create, delete::Delete, init::Init, retrieve::Retrieve, update::Update};

/// The specific command that CLInvoice should run.
#[derive(Clap, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Command
{
	/// Edit the CLInvoice configuration file in your default editor.
	///
	/// Setting your default editor depends on platform. On Unix-based systems, try setting
	/// `$EDITOR`.
	Config,

	/// `clinvoice create`
	Create(Create),

	/// `clinvoice delete`
	Delete(Delete),

	/// `clinvoice init`
	Init(Init),

	/// `clinvoice retrieve`
	Retrieve(Retrieve),

	/// `clinvoice update`
	Update(Update),
}
