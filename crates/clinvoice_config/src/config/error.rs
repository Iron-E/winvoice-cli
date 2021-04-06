use
{
	std::io,

	thiserror::Error,
};

#[derive(Debug, Error)]
pub enum Error
{
	#[cfg_attr(debug_assertions,      error("{0:?}"))]
	#[cfg_attr(not(debug_assertions), error("{0}"))]
	Io(#[from] io::Error),

	#[cfg_attr(debug_assertions,      error("{0:?}"))]
	#[cfg_attr(not(debug_assertions), error("{0}"))]
	TomlDe(#[from] toml::de::Error),

	#[cfg_attr(debug_assertions,      error("{0:?}"))]
	#[cfg_attr(not(debug_assertions), error("{0}"))]
	TomlSer(#[from] toml::ser::Error),
}

clinvoice_error::AliasResult!();
