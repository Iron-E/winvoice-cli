mod clinvoice;

use clinvoice::CLInvoice;
use structopt::StructOpt;

/// # Summary
///
/// The main method.
fn main()
{
	let args: CLInvoice = CLInvoice::from_args();
	println!("{:#?}", args);
}
