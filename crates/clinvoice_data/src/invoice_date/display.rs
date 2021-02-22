use
{
	super::InvoiceDate,
	std::fmt::{Display, Formatter, Result},
};

impl Display for InvoiceDate
{
	fn fmt(&self, formatter: &mut Formatter<'_>) -> Result
	{
		return write!(formatter, "Issued on {}, {}",
			self.issued, match self.paid
			{
				Some(p) => format!("Paid on {}", p),
				_ => "Outstanding".into(),
			},
		);
	}
}

#[cfg(test)]
mod tests
{
	use
	{
		super::InvoiceDate,
		crate::chrono::Utc,
		std::time::Instant,
	};

	#[test]
	fn test_display()
	{
		let start = Instant::now();

		let mut date = InvoiceDate
		{
			issued: Utc::now(),
			paid: None,
		};

		assert_eq!(format!("{}", date), format!("Issued on {}, Outstanding", date.issued));

		date = InvoiceDate
		{
			issued: Utc::now(),
			paid: Some(Utc::now()),
		};

		assert_eq!(format!("{}", date), format!("Issued on {}, Paid on {}", date.issued, date.paid.unwrap()));

		println!("\n>>>>> InvoiceDate test_display {}us <<<<<\n", Instant::now().duration_since(start).as_micros());
	}
}
