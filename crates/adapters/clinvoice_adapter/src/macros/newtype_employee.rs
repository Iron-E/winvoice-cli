#[macro_export]
macro_rules! NewtypeEmployee
{
	($name:ident, $($life:lifetime)*) =>
	{
		use clinvoice_data::Employee;

		/// # Summary
		///
		/// Wrapper around [`Employee`].
		pub struct $name<$($life),*> (Employee<$($life),*>) where
			'email : 'contact_info,
			'phone : 'contact_info,
		;

		impl<$($life),*> From<Employee<$($life),*>> for $name<$($life),*> where
			 'email : 'contact_info,
			 'phone : 'contact_info,
		{
			fn from(employee: Employee<$($life),*>) -> Self
			{
				return $name (employee);
			}
		}
	};
}
