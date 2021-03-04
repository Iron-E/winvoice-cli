use
{
	std::io,
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

clinvoice_error::FromError!(Io, io::Error);
clinvoice_error::FromError!(TomlDe, toml::de::Error);
clinvoice_error::FromError!(TomlSer, toml::ser::Error);
clinvoice_error::AliasResult!();
