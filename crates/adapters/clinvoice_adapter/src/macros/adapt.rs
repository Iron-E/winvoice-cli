/// # Summary
///
/// `Adapt!` is a marcro which allows quick generation of wrapper types necessary to implement
/// kJ"
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
#[macro_export]
macro_rules! Adapt
{
	(Employee => $name: ident) =>
	{
		clinvoice_adapter::AdaptEmployee!($name, 'emp, 'store);
	};

	(Job => $name: ident) =>
	{
		clinvoice_adapter::AdaptJob!($name, 'job, 'store);
	};

	(Location => $name: ident) =>
	{
		clinvoice_adapter::AdaptLocation!($name, 'loc, 'store);
	};

	(Organization => $name: ident) =>
	{
		clinvoice_adapter::AdaptOrganization!($name, 'org, 'store);
	};

	(Person => $name: ident) =>
	{
		clinvoice_adapter::AdaptPerson!($name, 'per, 'store);
	};
}
