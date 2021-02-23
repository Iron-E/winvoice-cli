pub mod create;
pub mod retrieve;

use
{
	create::Create,
	retrieve::Retrieve,
	crate::runnable::Runnable,
	clinvoice_adapter::Store,
	structopt::StructOpt,
};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructOpt)]
#[structopt(name="clinvoice", about="CLInvoice is a tool to help with invoicing from the command line!")]
pub struct App
{
	#[structopt(about="Select retrieved entities for deletion", default_value="default", long, short)]
	pub store: String,

	#[structopt(subcommand)]
	pub command: AppCommand,
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructOpt)]
pub enum AppCommand
{
	Create(Create),

	Retrieve(Retrieve),
}

impl<'pass, 'path, 'user> Runnable<'pass, 'path, 'user> for App
{
	fn run(self, store: Store<'pass, 'path, 'user>)
	{
		match self.command
		{
			AppCommand::Create(cmd) => cmd.run(store),
			AppCommand::Retrieve(cmd) => cmd.run(store),
		};
	}
}
