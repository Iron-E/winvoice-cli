use core::fmt::{Display, Formatter, Result};

use chrono::{DateTime, Local};

use super::Job;

impl Display for Job
{
	fn fmt(&self, formatter: &mut Formatter) -> Result
	{
		write!(
			formatter,
			"Job №{} for {}: {} – ",
			self.id,
			self.client.name,
			DateTime::<Local>::from(self.date_open).naive_local(),
		)?;

		match self.date_close
		{
			Some(date) => writeln!(formatter, "{}", DateTime::<Local>::from(date).naive_local()),
			_ => writeln!(formatter, "Current"),
		}?;

		/// One indent in, with a newline.
		const NEWLINE_INDENT: &str = "\n\t";

		/// Two indents in, with a newline.
		const NEWLINE_TWO_INDENTS: &str = "\n\t\t";

		// NOTE: we use `write` from here on out because it isn't certain which call will be the last

		write!(
			formatter,
			"\tInvoice:{NEWLINE_TWO_INDENTS}{}",
			self.invoice.to_string().replace('\n', NEWLINE_TWO_INDENTS)
		)?;

		if !self.objectives.is_empty()
		{
			write!(
				formatter,
				"{NEWLINE_INDENT}Objectives:{NEWLINE_TWO_INDENTS}{}",
				self.objectives.replace('\n', NEWLINE_TWO_INDENTS)
			)?;
		}

		if !self.notes.is_empty()
		{
			write!(
				formatter,
				"{NEWLINE_INDENT}Notes:{NEWLINE_TWO_INDENTS}{}",
				self.notes.replace('\n', NEWLINE_TWO_INDENTS)
			)?;
		}

		Ok(())
	}
}

#[cfg(test)]
mod tests
{
	use core::time::Duration;

	use chrono::Utc;
	use clinvoice_finance::{Currency, Money};

	use super::{DateTime, Job, Local};
	use crate::{Invoice, Location, Organization};

	#[test]
	fn display()
	{
		let earth_view = Location {
			id: 0,
			name: "Earth".into(),
			outer: None,
		};

		let create_job_view = Job {
			client: Organization {
				id: 0,
				location: earth_view.clone(),
				name: "Big Old Test".into(),
			},
			date_close: Some(Utc::today().and_hms(23, 59, 59)),
			date_open: Utc::now(),
			id: 0,
			increment: Duration::from_secs(900),
			invoice: Invoice {
				date: None,
				hourly_rate: Money::new(20_00, 2, Currency::Usd),
			},
			notes: "Remember not to work with these guys again!".into(),
			objectives: "Get into the mainframe, or something like that.\nClean the drawer.".into(),
		};

		assert_eq!(
			format!("{create_job_view}"),
			format!(
				"Job №{} for Big Old Test: {} – {}
	Invoice:
		Hourly Rate: 20.00 USD
		Status: Not issued
	Objectives:
		Get into the mainframe, or something like that.
		Clean the drawer.
	Notes:
		Remember not to work with these guys again!",
				create_job_view.id,
				DateTime::<Local>::from(create_job_view.date_open).naive_local(),
				DateTime::<Local>::from(create_job_view.date_close.unwrap()).naive_local(),
			),
		);
	}
}
