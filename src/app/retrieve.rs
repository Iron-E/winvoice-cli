use
{
	clinvoice_adapter::{DynamicResult, Store},
	structopt::StructOpt,
};

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructOpt)]
#[structopt(about="Retrieve information that was recorded with CLInvoice")]
pub(super) struct Retrieve
{
	#[structopt(about="Select retrieved entities for deletion", long, short)]
	pub delete: bool,

	#[structopt(about="Select retrieved entities for data updating", long, short)]
	pub update: bool,

	#[structopt(subcommand)]
	pub command: RetrieveCommand,
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructOpt)]
pub(super) enum RetrieveCommand
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

impl Retrieve
{
	pub(super) fn run(self, store: Store<'_, '_, '_>) -> DynamicResult<()>
	{
		todo!()
	}
}
