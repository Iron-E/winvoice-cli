use structopt::StructOpt;

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructOpt)]
#[structopt(name="create", about="Record information information with CLInvoice")]
pub enum Create
{
	#[structopt(name="employee", about="Create a new employee record")]
	Employee
	{
	},

	#[structopt(name="job", about="Create a new job record")]
	Job
	{
	},

	#[structopt(name="location", about="Create a new location record")]
	Location
	{
	},

	#[structopt(name="organization", about="Create a new organization record")]
	Organization
	{
	},

	#[structopt(name="person", about="Create a new organization record")]
	Person
	{
	},
}
