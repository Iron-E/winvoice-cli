mod from_io_error;
mod from_toml_de_error;
mod from_toml_ser_error;

use
{
	std::{io, result::Result as StdResult},
	snafu::Snafu,
};

#[derive(Debug, Snafu)]
pub enum Error
{
	#[cfg_attr(debug_assertions,      snafu(display("{:?}", err)))]
	#[cfg_attr(not(debug_assertions), snafu(display("{}",   err)))]
	Io {err: io::Error},

	#[cfg_attr(debug_assertions,      snafu(display("{:?}", err)))]
	#[cfg_attr(not(debug_assertions), snafu(display("{}",   err)))]
	TomlDe {err: toml::de::Error},

	#[cfg_attr(debug_assertions,      snafu(display("{:?}", err)))]
	#[cfg_attr(not(debug_assertions), snafu(display("{}",   err)))]
	TomlSer {err: toml::ser::Error},
}

pub type Result<T> = StdResult<T, Error>;
