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

		return write!(formatter, "Invoice Status: {}", match &self.date
		{
			Some(date) => date.to_string(),
			_ => "Not issued.".into(),
		});
	}
}

#[cfg(test)]
mod tests
{
	use
	{
		super::Invoice,
		crate::{chrono::Utc, Decimal, InvoiceDate, Money},
		std::time::Instant,
	};

	#[test]
	fn test_display()
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
Invoice Status: Issued on {}, Outstanding",
				invoice.date.unwrap().issued
			),
		);
		println!("\n>>>>> Invoice test_display {}us <<<<<\n", Instant::now().duration_since(start).as_micros());
	}
}
