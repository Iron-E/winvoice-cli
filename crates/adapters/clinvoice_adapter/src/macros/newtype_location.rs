#[macro_export]
macro_rules! newtype_location
{
	($name:ident) =>
	{
		use clinvoice_data::Location;

		/// # Summary
		///
		/// Wrapper around [`Employee`].
		pub struct $name<'name> (Location<'name>);

		impl<'name> From<Location<'name>> for $name<'name>
		{
			fn from(location: Location<'name>) -> Self
			{
				return $name (location);
			}
		}
	}
}
