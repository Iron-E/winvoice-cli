use
{
	core::fmt::{Display, Formatter, Result},

	super::Invoice,
};

impl Display for Invoice
{
	fn fmt(&self, formatter: &mut Formatter) -> Result
	{
		writeln!(formatter, "Hourly Rate: {}", self.hourly_rate)?;

		write!(formatter, "Status: {}", match &self.date
		{
			Some(date) => date.to_string(),
			_ => "Not issued".into(),
		})
	}
}

#[cfg(test)]
mod tests
{
	use
	{
		std::time::Instant,

		super::Invoice,
		crate::{Decimal, InvoiceDate, Money},

		chrono::{DateTime, Local, Utc},
	};

	#[test]
	fn display()
	{
		let invoice = Invoice
		{
			date: Some(InvoiceDate
			{
				issued: Utc::now(),
				paid: None,
			}),
			hourly_rate: Money::new(Decimal::new(1000, 2), "USD"),
		};

		let start = Instant::now();
		assert_eq!(
			format!("{}", invoice),
			format!(
"Hourly Rate: 10.00 USD
Status: Issued on {}; Outstanding",
				DateTime::<Local>::from(invoice.date.unwrap().issued),
			),
		);
		println!("\n>>>>> Invoice::fmt {}us <<<<<\n", Instant::now().duration_since(start).as_micros());
	}
}
