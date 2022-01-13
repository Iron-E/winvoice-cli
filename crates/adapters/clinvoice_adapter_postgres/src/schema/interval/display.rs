use core::fmt::{Display, Formatter, Result};

use super::PgInterval;
use humantime::Duration;

impl Display for PgInterval
{
	fn fmt(&self, f: &mut Formatter<'_>) -> Result
	{
		write!(f, "{}", Duration::from(self.0))
	}
}
