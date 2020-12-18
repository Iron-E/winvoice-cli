pub struct Location<'name, 'outer> where 'outer : 'name
{
	pub outer: Option<&'outer Self>,
	pub name: &'name str,
}
