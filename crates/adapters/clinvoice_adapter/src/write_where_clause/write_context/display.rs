use core::fmt::{Display, Formatter, Result};

use super::WriteContext;

impl Display for WriteContext
{
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		match self
		{
			WriteContext::AfterWhereCondition => write!(f, " AND"),
			WriteContext::BeforeWhereClause => write!(f, " WHERE"),
			WriteContext::InWhereCondition => Ok(()),
		}
	}
}
