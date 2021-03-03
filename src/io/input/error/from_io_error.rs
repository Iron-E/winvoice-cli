use std::io::Error;

impl From<Error> for super::Error
{
	fn from(err: Error) -> Self
	{
		Self::Io {err}
	}
}
