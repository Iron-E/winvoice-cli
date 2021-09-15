/// # Summary
///
/// `AdaptLocation!` is a marcro which allows quick generation of [`Location`](clinvoice_data::Location) wrapper types necessary to implement
/// `Crud` traits on a CLInvoice adapter.
///
/// # Examples
///
/// ```rust
/// clinvoice_adapter::AdaptLocation!(PostgresLocation<'loc, sqlx::PgPool>);
/// ```
#[macro_export]
macro_rules! AdaptLocation
{
	($name:ident<$life:lifetime, $Pool:ty>) =>
	{
		use clinvoice_data::Location;

		/// # Summary
		///
		/// A wrapper around [`Location`] with a [`Store`] that points to its location.
		#[derive(Clone, Debug)]
		pub struct $name<$life>
		{
			pub location: &$life Location,
			pub pool: &$life $Pool,
		}
	}
}
