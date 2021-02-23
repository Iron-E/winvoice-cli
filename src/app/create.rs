use
{
	crate::runnable::Runnable,
	clinvoice_adapter::Store,
	structopt::StructOpt,
};

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructOpt)]
#[structopt(about="Record information information with CLInvoice")]
pub enum Create
{
	#[structopt(about="Create a new employee record")]
	Employee
	{
	},

	#[structopt(about="Create a new job record")]
	Job
	{
	},

	#[structopt(about="Create a new location record")]
	Location
	{
	},

	#[structopt(about="Create a new organization record")]
	Organization
	{
	},

	#[structopt(about="Create a new organization record")]
	Person
	{
	},
}

impl<'pass, 'path, 'user> Runnable<'pass, 'path, 'user> for Create
{
	fn run(self, store: Store<'pass, 'path, 'user>)
	{
	}
}
