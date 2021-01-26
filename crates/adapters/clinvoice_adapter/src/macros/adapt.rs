#[macro_export]
/// # Summary
///
/// `Adapt!` is a marcro which allows quick generation of wrapper types necessary to implement
/// `Crud` traits on a CLInvoice adapter.
///
/// # Parameters
///
/// There are several arms to this macro:
///
/// * `Employee => $name:ident` creates new [`Employee`](clinvoice_data::Employee) wrapper.
/// * `Job => $name:ident` creates new [`Job`](clinvoice_data::Job) wrapper.
/// * `Location => $name:ident` creates new [`Location`](clinvoice_data::Location) wrapper.
/// * `Organization => $name:ident` creates new [`Organization`](clinvoice_data::Organization) wrapper.
/// * `Person => $name:ident` creates new [`Person`](clinvoice_data::Person) wrapper.
///
/// # Examples
///
/// ```rust
/// clinvoice_adapter::Adapt!(Employee => FooEmployee);
/// ```
macro_rules! Adapt
{
	(Employee => $name: ident) =>
	{
		clinvoice_adapter::AdaptEmployee!($name, 'email 'phone 'title, 'pass 'path 'user);
	};

	(Job => $name: ident) =>
	{
		clinvoice_adapter::AdaptJob!($name, 'objectives 'notes 'work_notes, 'pass 'path 'user);
	};

	(Location => $name: ident) =>
	{
		clinvoice_adapter::AdaptLocation!($name, 'name, 'pass 'path 'user);
	};

	(Organization => $name: ident) =>
	{
		clinvoice_adapter::AdaptOrganization!($name, 'name, 'pass 'path 'user);
	};

	(Person => $name: ident) =>
	{
		clinvoice_adapter::AdaptPerson!($name, 'email 'name 'phone, 'pass 'path 'user);
	};
}
