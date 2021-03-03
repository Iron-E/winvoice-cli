use toml::de::Error;

impl From<Error> for super::Error
{
	fn from(err: Error) -> Self
	{
		Self::TomlDe {err}
	}
}
