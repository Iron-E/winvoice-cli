use super::{Invoice, MockMoney};
use std::hash::{Hash, Hasher};

impl Hash for Invoice
{
	fn hash<H>(&self, state: &mut H) where H : Hasher
	{
		for date in &[self.date_issued, self.date_paid]
		{
			match date
			{
				Some(d) => d.to_string().as_bytes().hash(state),
				None => ().hash(state),
			};
		}

		MockMoney::from(&self.hourly_rate).hash(state);
	}
}
