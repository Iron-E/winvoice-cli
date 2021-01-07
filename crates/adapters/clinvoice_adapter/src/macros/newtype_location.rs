#[macro_export]
macro_rules! NewtypeLocation
{
	($name: ident, $($life: lifetime)*) =>
	{
		use clinvoice_data::Location;

		/// # Summary
		///
		/// Wrapper around [`Employee`].
		pub struct $name<$($life),*> (Location<$($life),*>);

		impl<$($life),*> From<Location<$($life),*>> for $name<$($life),*>
		{
			fn from(location: Location<$($life),*>) -> Self
			{
				return $name (location);
			}
		}
	}
}
