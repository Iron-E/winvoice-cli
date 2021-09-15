/// # Summary
///
/// `AdaptOrganization!` is a marcro which allows quick generation of [`Organization`](clinvoice_data::Organization) wrapper types necessary to implement
/// `Crud` traits on a CLInvoice adapter.
///
/// # Examples
///
/// ```rust
/// clinvoice_adapter::AdaptOrganization!(PostgresOrganization<'org, sqlx::PgPool>);
/// ```
#[macro_export]
macro_rules! AdaptOrganization
{
	($name:ident<$life:lifetime, $Pool:ty>) =>
	{
		use clinvoice_data::Organization;

		/// # Summary
		///
		/// A wrapper around [`Organization`] with a [`Store`] that points to its location.
		#[derive(Clone, Debug)]
		pub struct $name<$life>
		{
			pub organization: &$life Organization,
			pub pool: &$life $Pool,
		}
	}
}
