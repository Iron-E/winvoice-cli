use
{
	crate::{Config, StructOpt},
	clinvoice_adapter::{DynamicResult, Store},
};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructOpt)]
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
	fn create_employee() -> DynamicResult<()>
	{
		todo!()
	}

	fn create_job() -> DynamicResult<()>
	{
		todo!()
	}

	fn create_location() -> DynamicResult<()>
	{
		todo!()
	}

	fn create_organization() -> DynamicResult<()>
	{
		todo!()
	}

	fn create_person(store: Store) -> DynamicResult<()>
	{
		todo!()
	}

	pub(super) fn run(self, config: Config, store_name: &str) -> DynamicResult<()>
	{
		todo!()
	}
}
