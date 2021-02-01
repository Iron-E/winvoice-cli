use clinvoice_data::Id;
use snafu::Snafu;

/// # Summary
///
/// Errors for the data
#[derive(Debug, Snafu)]
pub enum Error
{
	/// # Summary
	///
	/// Some reference to an `id` was expected, but none was found.
	#[snafu(display("A reference to ID #{} was expected, but `None` was found.", id))]
	DataIntegrity {id: Id},
}
