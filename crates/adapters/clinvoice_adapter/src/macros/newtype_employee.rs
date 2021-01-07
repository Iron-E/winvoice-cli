#[macro_export]
macro_rules! newtype_employee
{
	($name:ident) =>
	{
		use clinvoice_data::Employee;

		/// # Summary
		///
		/// Wrapper around [`Employee`].
		pub struct $name<'contact_info, 'email, 'phone> (Employee<'contact_info, 'email, 'phone>);

		impl<'contact_info, 'email, 'phone> From<Employee<'contact_info, 'email, 'phone>>
		for $name<'contact_info, 'email, 'phone>
		{
			fn from(employee: Employee<'contact_info, 'email, 'phone>) -> Self
			{
				return $name (employee);
			}
		}
	}
}
