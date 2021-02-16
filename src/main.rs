mod clinvoice;

use clinvoice::CLInvoice;
use structopt::StructOpt;

/// # Summary
///
/// The main method.
fn main()
{
	println!("{:#?}", CLInvoice::from_args());
}
