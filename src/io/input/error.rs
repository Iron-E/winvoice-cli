mod from_io_error;
mod from_toml_de_error;
mod from_toml_ser_error;

use
{
	std::{io, result::Result as StdResult},
	snafu::Snafu
};

/// # Summary
///
/// [`Error`](std::error::Error)s referencing [`Store`](crate::Store)s and [`Adapters`].
#[derive(Debug, Snafu)]
pub enum Error
{
	#[cfg_attr(debug_assertions,      snafu(display("{:?}", err)))]
	#[cfg_attr(not(debug_assertions), snafu(display("{}",   err)))]
	Io {err: io::Error},

	/// # Summary
	///
	/// An entity needed to be edited in order to be valid, but the user did not edit it.
	#[snafu(display("The passed in entity was not edited by the user."))]
	NotEdited,

	#[cfg_attr(debug_assertions,      snafu(display("{:?}", err)))]
	#[cfg_attr(not(debug_assertions), snafu(display("{}",   err)))]
	TomlDe {err: toml::de::Error},

	#[cfg_attr(debug_assertions,      snafu(display("{:?}", err)))]
	#[cfg_attr(not(debug_assertions), snafu(display("{}",   err)))]
	TomlSer {err: toml::ser::Error},
}

pub type Result<T> = StdResult<T, Error>;
