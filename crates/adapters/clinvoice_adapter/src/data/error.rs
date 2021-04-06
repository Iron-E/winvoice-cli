use
{
	clinvoice_data::Id,

	thiserror::Error,
};

/// # Summary
///
/// Errors for the data
#[derive(Copy, Clone, Debug, Eq, Error, Hash, Ord, PartialEq, PartialOrd)]
pub enum Error
{
	/// # Summary
	///
	/// Some reference to an `id` was expected, but none was found.
	#[error("A reference to ID #{0} was expected, but `None` was found")]
	DataIntegrity(Id),

	/// # Summary
	///
	/// At least one of some entity is necessary to perform an operation, but none were found.
	#[error("There must be at least one `{0}` before this operation can be performed")]
	NoData(&'static str),
}

clinvoice_error::AliasResult!();
