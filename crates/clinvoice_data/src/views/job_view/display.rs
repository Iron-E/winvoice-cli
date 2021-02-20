use
{
	super::JobView,
	crate::Job,
	std::fmt::{Display, Formatter, Result},
};

impl Display for JobView
{
	fn fmt(&self, formatter: &mut Formatter<'_>) -> Result
	{
		writeln!(formatter, "Job #{} for {}: {} – {}",
			self.id,
			self.client.name,
			self.date_open,
			match self.date_close
			{
				Some(date) => date.to_string(),
				_ => "Current".into(),
			},
		)?;

		writeln!(formatter, "{}", self.invoice)?;
		writeln!(formatter, "Notes:\n\t{}", self.notes.replace('\n', "\n\t"))?;
		writeln!(formatter, "Objectives:\n\t{}", self.objectives.replace('\n', "\n\t"))?;
		writeln!(formatter, "Timesheets:")?;
		self.timesheets.iter().try_for_each(|t| writeln!(formatter, "{}", t.to_string().replace('\n', "\n\t")))?;

		return write!(formatter, "Total Amount Owed: {}", Job::from(self.clone()).total());
	}
}
