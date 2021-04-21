use
{
	core::fmt::{Display, Formatter, Result},
	super::TimeCommand,
};

impl Display for TimeCommand
{
	fn fmt(&self, formatter: &mut Formatter<'_>) -> Result
	{
		match self
		{
			Self::Start => write!(formatter, "start"),
			Self::Stop => write!(formatter, "stop"),
		}
	}
}
