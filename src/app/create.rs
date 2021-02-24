use
{
	crate::{Config, StructOpt},
	clinvoice_adapter::DynamicResult,
};

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructOpt)]
#[structopt(about="Record information information with CLInvoice")]
pub(super) enum Create
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

impl Create
{
	pub(super) fn run(self, config: Config<'_, '_, '_, '_, '_, '_>, store_name: &str) -> DynamicResult<()>
	{
		todo!()
	}
}
