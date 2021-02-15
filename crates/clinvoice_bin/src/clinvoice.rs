pub mod create;
pub mod retrieve;

use
{
	create::Create,
	retrieve::Retrieve,
	structopt::StructOpt,
};

/// # Summary
///
/// The entry-point for the `clinvoice` program.
#[derive(Debug, StructOpt)]
#[structopt(name="clinvoice", about="Invoice from the command line!")]
pub enum CLInvoice
{
	#[structopt(flatten)]
	Create(Create),

	#[structopt(flatten)]
	Retrieve(Retrieve),
}
