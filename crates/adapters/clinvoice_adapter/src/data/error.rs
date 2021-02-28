use
{
	clinvoice_data::Id,
	snafu::Snafu,
};

/// # Summary
///
/// Errors for the data
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Snafu)]
pub enum Error
{
	/// # Summary
	///
	/// Some reference to an `id` was expected, but none was found.
	#[snafu(display("A reference to ID #{} was expected, but `None` was found.", id))]
	DataIntegrity {id: Id},
}
