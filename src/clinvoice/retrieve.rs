use structopt::StructOpt;

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructOpt)]
#[structopt(name="retrieve", about="Retrieve information that was recorded with CLInvoice")]
pub struct Retrieve
{
	#[structopt(short, long, about="Select retrieved entities for deletion")]
	pub delete: bool,

	#[structopt(short, long, about="Select retrieved entities for data updating")]
	pub update: bool,

	#[structopt(subcommand)]
	pub command: RetrieveCommand,
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructOpt)]
pub enum RetrieveCommand
{
	#[structopt(name="employee", about="Retrieve existing records about employees")]
	Employee
	{
	},

	#[structopt(name="job", about="Retrieve existing records about job")]
	Job
	{
	},

	#[structopt(name="location", about="Retrieve existing records about locations")]
	Location
	{
	},

	#[structopt(name="organization", about="Retrieve existing records about organizations")]
	Organization
	{
	},

	#[structopt(name="person", about="Retrieve existing records about people")]
	Person
	{
	},
}
