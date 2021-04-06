use
{
	std::io,

	thiserror::Error
};

/// # Summary
///
/// [`Error`](std::error::Error)s referencing [`Store`](crate::Store)s and [`Adapters`].
#[derive(Debug, Error)]
pub enum Error
{
	#[cfg_attr(debug_assertions,      error("{0:?}"))]
	#[cfg_attr(not(debug_assertions), error("{0}"))]
	Io(#[from] io::Error),

	/// # Summary
	///
	/// An entity needed to be edited in order to be valid, but the user did not edit it.
	#[error("The text was not edited")]
	NotEdited,

	#[cfg_attr(debug_assertions,      error("{0:?}"))]
	#[cfg_attr(not(debug_assertions), error("{0}"))]
	TomlDe(#[from] toml::de::Error),

	#[cfg_attr(debug_assertions,      error("{0:?}"))]
	#[cfg_attr(not(debug_assertions), error("{0}"))]
	TomlSer(#[from] toml::ser::Error),
}

clinvoice_error::AliasResult!();
