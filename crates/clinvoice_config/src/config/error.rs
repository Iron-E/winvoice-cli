mod from_io_error;
mod from_toml_error;

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
	Toml {err: toml::ser::Error},
}

pub type Result<T> = StdResult<T, Error>;
