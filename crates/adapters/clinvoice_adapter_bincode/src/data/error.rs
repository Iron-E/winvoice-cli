use
{
	std::io,

	clinvoice_adapter::data,

	thiserror::Error,
};

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
