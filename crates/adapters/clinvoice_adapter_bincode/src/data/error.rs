use std::io;

use clinvoice_adapter::data;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error
{
	#[error("{0}")]
	Bincode(#[from] bincode::Error),

	#[error("{0}")]
	Data(#[from] data::Error),

	#[error("{0}")]
	Io(#[from] io::Error),
}

clinvoice_error::AliasResult!();
