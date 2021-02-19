use
{
	super::Invoice,
	std::fmt::{Display, Formatter, Result},
};

impl Display for Invoice
{
	fn fmt(&self, formatter: &mut Formatter<'_>) -> Result
	{
		writeln!(formatter, "Hourly Rate: {}", self.hourly_rate)?;

		if let Some(d) = &self.date
		{
			return write!(formatter, "Invoice Status: Issued on {}, {}",
				d.issued, match d.paid
				{
					Some(p) => format!("Paid on {}", p),
					_ => "Outstanding".into(),
				},
			);
		}

		return write!(formatter, "Invoice Status: Not sent");
	}
}



