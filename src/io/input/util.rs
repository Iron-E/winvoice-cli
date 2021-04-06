pub mod contact;
pub mod employee_status;
pub mod location;
pub mod organization;
pub mod person;

use
{
	clinvoice_data::views::RestorableSerde,

	serde::{Deserialize, Serialize},
};

#[derive(Debug, Deserialize, Serialize)]
struct SerdeWrapper<T> { value: T }

impl<R> RestorableSerde for SerdeWrapper<R> where
	R : RestorableSerde,
{
	fn restore(&mut self, original: &Self)
	{
		self.value.restore(&original.value);
	}
}
