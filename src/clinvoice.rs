pub mod create;
pub mod retrieve;

use
{
	create::Create,
	retrieve::Retrieve,
	structopt::StructOpt,
};

#[derive(Debug, StructOpt)]
#[structopt(name="clinvoice", about="CLInvoice is a tool to help with invoicing from the command line!")]
pub enum CLInvoice
{
	Create(Create),

	Retrieve(Retrieve),
}
