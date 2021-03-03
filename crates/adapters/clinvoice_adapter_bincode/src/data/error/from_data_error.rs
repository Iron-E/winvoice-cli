use clinvoice_adapter::data::Error;

impl From<Error> for super::Error
{
	fn from(err: Error) -> Self
	{
		Self::Data {err}
	}
}
