pub mod create;
pub mod retrieve;

use
{
	create::Create,
	retrieve::Retrieve,
	clinvoice_adapter::{DynamicResult, Store},
	structopt::StructOpt,
};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructOpt)]
#[structopt(name="clinvoice", about="CLInvoice is a tool to help with invoicing from the command line!")]
pub struct App
{
	#[structopt(about="Select retrieved entities for deletion", default_value="default", long, short)]
	store: String,

	#[structopt(subcommand)]
	command: AppCommand,
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructOpt)]
enum AppCommand
{
	Create(Create),

	Retrieve(Retrieve),
}

impl App
{
	pub fn run(self, store: Store<'_, '_, '_>) -> DynamicResult<()>
	{
		return Ok(match self.command
		{
			AppCommand::Create(cmd) => cmd.run(store)?,
			AppCommand::Retrieve(cmd) => cmd.run(store)?,
		});
	}
}
