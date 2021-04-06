use
{
	core::fmt::{Display, Formatter, Result},

	super::InvoiceDate,
};

impl Display for InvoiceDate
{
	fn fmt(&self, formatter: &mut Formatter) -> Result
	{
		write!(formatter, "Issued on {}, {}",
			self.issued, match self.paid
			{
				Some(p) => format!("Paid on {}", p),
				_ => "Outstanding".into(),
			},
		)
	}
}

#[cfg(test)]
mod tests
{
	use
	{
		std::time::Instant,

		super::InvoiceDate,
		crate::chrono::Utc,
	};

	#[test]
	fn display()
	{
		let date = InvoiceDate
		{
			issued: Utc::now(),
			paid: None,
		};

		let other_date = InvoiceDate
		{
			issued: Utc::now(),
			paid: Some(Utc::now()),
		};

		let start = Instant::now();
		assert_eq!(format!("{}", date), format!("Issued on {}, Outstanding", date.issued));
		assert_eq!(format!("{}", other_date), format!("Issued on {}, Paid on {}", other_date.issued, other_date.paid.unwrap()));
		println!("\n>>>>> InvoiceDate::fmt {}us <<<<<\n", Instant::now().duration_since(start).as_micros() / 2);
	}
}
