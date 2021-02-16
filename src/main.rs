mod clinvoice;
mod io;

use
{
	clinvoice::CLInvoice,
	structopt::StructOpt,
};

/// # Summary
///
/// The main method.
fn main()
{
	let args: CLInvoice = CLInvoice::from_args();
	println!("{:#?}", args);
}
