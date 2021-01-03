use crate::Location;

pub enum Contact<'addr, 'email>
{
	Address(Location<'addr>),
	Email(&'email str),
	Phone(u16),
}
