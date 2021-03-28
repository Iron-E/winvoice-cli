use super::{Invoice, invoice_date_default};

impl Default for Invoice<'_>
{
	fn default() -> Self
	{
		Self
		{
			date: invoice_date_default(),
			..Default::default()
		}
	}
}
