use super::StoreArgs;

impl From<&str> for StoreArgs
{
	fn from(s: &str) -> Self
	{
		Self { store: s.to_owned() }
	}
}
