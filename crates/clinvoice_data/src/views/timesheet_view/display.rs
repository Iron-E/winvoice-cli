use
{
	super::TimesheetView,
	std::fmt::{Display, Formatter, Result},
};

impl Display for TimesheetView
{
	fn fmt(&self, formatter: &mut Formatter<'_>) -> Result
	{
		writeln!(formatter, "{} from {}: {} – {}",
			self.employee.person.name,
			self.employee.organization.name,
			self.time_begin,
			match self.time_end
			{
				Some(time) => time.to_string(),
				_ => "Current".into(),
			},
		)?;

		return write!(formatter, "\nWork Notes:\n\t{}", self.work_notes.replace('\n', "\n\t"));
	}
}

// TODO
