use toml::ser::Error;

impl From<Error> for super::Error
{
	fn from(err: Error) -> Self
	{
		Self::Toml {err}
	}
}
