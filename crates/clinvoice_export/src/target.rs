mod error;
mod from_str;

pub use error::{Error, Result};

/// # Summary
///
/// A target for exporting.
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Target
{
	/// # Summary
	///
	/// The markdown target. Exports to a `.md` file.
	#[cfg(feature="markdown")]
	Markdown,
}
