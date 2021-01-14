#[macro_export]
/// # Summary
///
/// `Newtype!` is a marcro which allows quick generation of wrapper types necessary to implement
/// `Crud` traits on a CLInvoice adapter.
///
/// # Parameters
///
/// There are several arms to this macro:
///
/// * `Employee => $name:ident` creates new [`Employee`](clinvoice_data::Employee) wrapper.
/// * `Job => $name:ident` creates new [`Job`](clinvoice_data::Job) wrapper.
/// * `Location => $name:ident` creates new [`Job`](clinvoice_data::Job) wrapper.
/// * `Organization => $name:ident` creates new [`Organization`](clinvoice_data::Organization) wrapper.
/// * `Person => $name:ident` creates new [`Person`](clinvoice_data::Person) wrapper.
///
/// # Examples
///
/// ```rust
/// clinvoice_adapter::Newtype!(Employee => FooEmployee);
/// ```
macro_rules! Newtype
{
	(Employee => $name: ident) =>
	{
		clinvoice_adapter::NewtypeEmployee!($name, 'contact_info 'email 'phone 'title);
	};

	(Job => $name: ident) =>
	{
		clinvoice_adapter::NewtypeJob!($name, 'objectives 'notes 'timesheets 'work_notes, TZone);
	};

	(Location => $name: ident) =>
	{
		clinvoice_adapter::NewtypeLocation!($name, 'name);
	};

	(Organization => $name: ident) =>
	{
		clinvoice_adapter::NewtypeOrganization!($name, 'name);
	};

	(Person => $name: ident) =>
	{
		clinvoice_adapter::NewtypePerson!($name, 'contact_info 'email 'name 'phone);
	};
}
