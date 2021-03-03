mod from_bincode_error;
mod from_data_error;
mod from_io_error;

use
{
	clinvoice_adapter::data,
	std::{io, result::Result as StdResult},
	snafu::Snafu,
};

#[derive(Debug, Snafu)]
pub enum Error
{
	#[cfg_attr(debug_assertions,      snafu(display("{:?}", err)))]
	#[cfg_attr(not(debug_assertions), snafu(display("{}",   err)))]
	Bincode {err: bincode::Error},

	#[cfg_attr(debug_assertions,      snafu(display("{:?}", err)))]
	#[cfg_attr(not(debug_assertions), snafu(display("{}",   err)))]
	Data {err: data::Error},

	#[cfg_attr(debug_assertions,      snafu(display("{:?}", err)))]
	#[cfg_attr(not(debug_assertions), snafu(display("{}",   err)))]
	Io {err: io::Error},
}

pub type Result<T> = StdResult<T, Error>;
