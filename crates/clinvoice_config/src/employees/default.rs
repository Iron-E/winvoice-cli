use super::{Employees, Id};

impl Default for Employees
{
	fn default() -> Self
	{
		Self {
			default_id: Default::default(),
		}
	}
}
