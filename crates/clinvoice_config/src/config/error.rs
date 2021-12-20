use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error
{
	#[error(transparent)]
	Io(#[from] io::Error),

	#[error(transparent)]
	TomlDe(#[from] toml::de::Error),

	#[error(transparent)]
	TomlSer(#[from] toml::ser::Error),
}

clinvoice_error::AliasResult!();
