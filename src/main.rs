mod clinvoice;

use clinvoice::{CLInvoice, create::Create};

/// # Summary
///
/// The main method.
fn main()
{
	println!("{:#?}", CLInvoice::Create(Create::Employee {}));
}
