/// # Summary
///
/// `AdaptPerson!` is a marcro which allows quick generation of [`Person`](clinvoice_data::Person) wrapper types necessary to implement
/// `Crud` traits on a CLInvoice adapter.
///
/// # Examples
///
/// ```rust
/// clinvoice_adapter::AdaptPerson!(PostgresPerson<'per, sqlx::PgPool>);
/// ```
#[macro_export]
macro_rules! AdaptPerson
{
	($name:ident<$life:lifetime, $Pool:ty>) =>
	{
		use clinvoice_data::Person;

		/// # Summary
		///
		/// A wrapper around [`Person`] with a [`Store`] that points to its location.
		#[derive(Clone, Debug)]
		pub struct $name<$life>
		{
			pub person: &$life Person,
			pub pool: &$life $Pool,
		}
	}
}
