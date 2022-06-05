use core::fmt::{Display, Formatter, Result};

use humantime::Duration;

use super::PgInterval;

impl Display for PgInterval
{
	fn fmt(&self, f: &mut Formatter<'_>) -> Result
	{
		write!(f, "{}", Duration::from(self.0))
	}
}
