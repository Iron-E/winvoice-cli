#[macro_export]
macro_rules! Newtype
{
	(Employee => $name: ident) =>
	{
		clinvoice_adapter::NewtypeEmployee!($name, 'contact_info 'email 'phone);
	};

	(Invoice => $name: ident) =>
	{
		clinvoice_adapter::NewtypeInvoice!($name, TZone);
	};

	(Job => $name: ident) =>
	{
		clinvoice_adapter::NewtypeJob!($name, 'objectives 'notes, TZone);
	};

	(Location => $name: ident) =>
	{
		clinvoice_adapter::NewtypeLocation!($name, 'name);
	};

	(Organization => $name: ident) =>
	{
		clinvoice_adapter::NewtypeOrganization!($name, 'name 'rep_title);
	};

	(Person => $name: ident) =>
	{
		clinvoice_adapter::NewtypePerson!($name, 'contact_info 'email 'name 'phone);
	};

	(Timesheet => $name: ident) =>
	{
		clinvoice_adapter::NewtypeTimesheet!($name, 'work_notes, TZone);
	};
}
