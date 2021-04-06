use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error
{
	#[error("The target '{0}' was not recognized")]
	InvalidTarget(String),
}

clinvoice_error::AliasResult!();
