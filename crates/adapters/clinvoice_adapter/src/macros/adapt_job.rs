/// # Summary
///
/// `AdaptJob!` is a marcro which allows quick generation of [`Job`](clinvoice_data::Job) wrapper types necessary to implement
/// `Crud` traits on a CLInvoice adapter.
///
/// # Examples
///
/// ```rust
/// clinvoice_adapter::AdaptJob!(PostgresJob<'job, sqlx::PgPool>);
/// ```
#[macro_export]
macro_rules! AdaptJob
{
	($name:ident<$life:lifetime, $Pool:ty>) =>
	{
		use clinvoice_data::Job;

		/// # Summary
		///
		/// A wrapper around [`Job`] with a [`Store`] that points to its location.
		#[derive(Clone, Debug)]
		pub struct $name<$life>
		{
			pub job: &$life Job,
			pub pool: &$life $Pool,
		}
	};
}
