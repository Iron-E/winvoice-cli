use super::StoreArgs;

impl<T> From<T> for StoreArgs
where
	T: Into<String>,
{
	fn from(t: T) -> Self
	{
		Self { store: t.into() }
	}
}
