mod app;
mod io;
mod runnable;

use
{
	app::App,
	structopt::StructOpt,
};

/// # Summary
///
/// The main method.
fn main()
{
	let clinvoice: App = App::from_args();
	println!("{:#?}", clinvoice);
}
