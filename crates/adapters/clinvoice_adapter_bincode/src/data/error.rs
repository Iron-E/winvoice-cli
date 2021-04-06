use
{
	std::io,

	clinvoice_adapter::data,

	thiserror::Error,
};

#[derive(Debug, Error)]
pub enum Error
{
	#[cfg_attr(debug_assertions,      error("{0:?}"))]
	#[cfg_attr(not(debug_assertions), error("{0}"))]
	Bincode(#[from] bincode::Error),

	#[cfg_attr(debug_assertions,      error("{0:?}"))]
	#[cfg_attr(not(debug_assertions), error("{0}"))]
	Data(#[from] data::Error),

	#[cfg_attr(debug_assertions,      error("{0:?}"))]
	#[cfg_attr(not(debug_assertions), error("{0}"))]
	Io(#[from] io::Error),
}

clinvoice_error::AliasResult!();
