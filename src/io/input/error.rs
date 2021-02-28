use snafu::Snafu;

/// # Summary
///
/// [`Error`](std::error::Error)s referencing [`Store`](crate::Store)s and [`Adapters`].
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Snafu)]
pub enum Error
{
	/// # Summary
	///
	/// An entity needed to be edited in order to be valid, but the user did not edit it.
	#[snafu(display("The passed in entity was not edited by the user.", adapter))]
	NotEdited,
}
