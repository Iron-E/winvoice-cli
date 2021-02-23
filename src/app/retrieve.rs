use
{
	crate::runnable::Runnable,
	clinvoice_adapter::Store,
	structopt::StructOpt,
};

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructOpt)]
#[structopt(about="Retrieve information that was recorded with CLInvoice")]
pub struct Retrieve
{
	#[structopt(about="Select retrieved entities for deletion", long, short)]
	pub delete: bool,

	#[structopt(about="Select retrieved entities for data updating", long, short)]
	pub update: bool,

	#[structopt(subcommand)]
	pub command: RetrieveCommand,
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructOpt)]
pub enum RetrieveCommand
{
	#[structopt(about="Retrieve existing records about employees")]
	Employee
	{
	},

	#[structopt(about="Retrieve existing records about job")]
	Job
	{
	},

	#[structopt(about="Retrieve existing records about locations")]
	Location
	{
	},

	#[structopt(about="Retrieve existing records about organizations")]
	Organization
	{
	},

	#[structopt(about="Retrieve existing records about people")]
	Person
	{
	},
}

impl<'pass, 'path, 'user> Runnable<'pass, 'path, 'user> for Retrieve
{
	fn run(self, store: Store<'pass, 'path, 'user>)
	{
	}
}
