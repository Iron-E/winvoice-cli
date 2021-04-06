use
{
	clinvoice_adapter::data,
	std::io,
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
