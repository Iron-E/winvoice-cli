/// # Summary
///
/// `AdaptEmployee!` is a marcro which allows quick generation of [`Employee`](clinvoice_data::Employee) wrapper types necessary to implement
/// `Crud` traits on a CLInvoice adapter.
///
/// # Examples
///
/// ```rust
/// clinvoice_adapter::AdaptEmployee!(PostgresEmployee<'emp, sqlx::PgPool>);
/// ```
#[macro_export]
macro_rules! AdaptEmployee
{
	($name:ident<$life:lifetime, $Pool:ty>) =>
	{
		use clinvoice_data::Employee;

		/// # Summary
		///
		/// A wrapper around [`Employee`] with a [`Store`] that points to its location.
		#[derive(Clone, Debug)]
		pub struct $name<$life>
		{
			pub employee: &$life Employee,
			pub pool: &$life $Pool,
		}
	};
}
