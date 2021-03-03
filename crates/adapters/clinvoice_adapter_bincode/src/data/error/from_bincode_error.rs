use bincode::Error;

impl From<Error> for super::Error
{
	fn from(err: Error) -> Self
	{
		Self::Bincode {err}
	}
}
