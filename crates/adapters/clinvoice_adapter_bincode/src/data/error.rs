use
{
	clinvoice_adapter::data,
	std::io,
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

clinvoice_error::FromError!(Bincode, bincode::Error);
clinvoice_error::FromError!(Data, data::Error);
clinvoice_error::FromError!(Io, io::Error);
clinvoice_error::AliasResult!();
