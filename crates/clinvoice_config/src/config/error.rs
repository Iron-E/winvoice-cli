use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error
{
	#[error("{0}")]
	Io(#[from] io::Error),

	#[error("{0}")]
	TomlDe(#[from] toml::de::Error),

	#[error("{0}")]
	TomlSer(#[from] toml::ser::Error),
}

clinvoice_error::AliasResult!();
