pub struct EmployerConfig<'address, 'email, 'name>
{
	address: &'address str,
	email: &'email str,
	name: &'name str,
}
