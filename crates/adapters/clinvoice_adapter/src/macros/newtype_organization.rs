#[macro_export]
macro_rules! NewtypeOrganization
{
	($name: ident, $($life: lifetime)*) =>
	{
		use clinvoice_data::Organization;

		/// # Summary
		///
		/// Wrapper around [`Employee`].
		pub struct $name<$($life),*> (Organization<$($life),*>);

		impl<$($life),*> From<Organization<$($life),*>> for $name<$($life),*>
		{
			fn from(organization: Organization<$($life),*>) -> Self
			{
				return $name (organization);
			}
		}
	}
}
