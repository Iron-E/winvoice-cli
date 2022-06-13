use std::io;

use thiserror::Error;

use crate::Adapters;

/// # Summary
///
/// [`Error`](std::error::Error)s referencing [`Store`](crate::Store)s and [`Adapters`].
#[derive(Debug, Error)]
pub enum Error
{
	#[error("Using this adapter requires the {0} feature")]
	FeatureNotFound(Adapters),

	#[error(transparent)]
	Io(#[from] io::Error),

	#[error(transparent)]
	TomlDe(#[from] toml::de::Error),

	#[error(transparent)]
	TomlSer(#[from] toml::ser::Error),
}

clinvoice_error::AliasResult!();
