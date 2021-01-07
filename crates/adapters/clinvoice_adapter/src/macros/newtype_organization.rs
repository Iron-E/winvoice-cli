#[macro_export]
macro_rules! newtype_organization
{
	($name:ident) =>
	{
		use clinvoice_data::Organization;

		/// # Summary
		///
		/// Wrapper around [`Employee`].
		pub struct $name<'name, 'rep_title> (Organization<'name, 'rep_title>);

		impl<'name, 'rep_title> From<Organization<'name, 'rep_title>> for $name<'name, 'rep_title>
		{
			fn from(organization: Organization<'name, 'rep_title>) -> Self
			{
				return $name (organization);
			}
		}
	}
}
