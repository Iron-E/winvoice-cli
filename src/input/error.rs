use std::io;

use clinvoice_adapter::data;
use serde_yaml as yaml;
use thiserror::Error;

/// # Summary
///
/// [`Error`](std::error::Error)s referencing [`Store`](crate::Store)s and [`Adapters`].
#[derive(Debug, Error)]
pub enum Error
{
	#[error("{0}")]
	Data(#[from] data::Error),

	#[error("{0}")]
	Io(#[from] io::Error),

	/// # Summary
	///
	/// An entity needed to be edited in order to be valid, but the user did not edit it.
	#[error("The text was not edited")]
	NotEdited,

	#[error("{0}")]
	Yaml(#[from] yaml::Error),
}

clinvoice_error::AliasResult!();
