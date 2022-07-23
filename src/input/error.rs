use std::io;

use clinvoice_config::Error as ConfigError;
use clinvoice_schema::RestoreError;
use serde_yaml as yaml;
use thiserror::Error;

/// An [`Error`](std::error::Error) for getting input from STDIO.
#[derive(Debug, Error)]
pub enum Error
{
	#[allow(missing_docs)]
	#[error(transparent)]
	Config(#[from] ConfigError),

	#[allow(missing_docs)]
	#[error(transparent)]
	Io(#[from] io::Error),

	#[allow(missing_docs)]
	#[error("There were no {0} values, and at least one was required")]
	NoData(String),

	#[allow(missing_docs)]
	#[error("The text was not edited")]
	NotEdited,

	#[allow(missing_docs)]
	#[error(transparent)]
	Restore(#[from] RestoreError),

	#[allow(missing_docs)]
	#[error(transparent)]
	Yaml(#[from] yaml::Error),
}

clinvoice_error::AliasResult!();
