use core::fmt::{Display, Formatter, Result};

use crate::fmt::sql;

use super::WriteContext;

impl Display for WriteContext
{
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		match self
		{
			WriteContext::AcceptingAnotherWhereCondition => sql::AND.fmt(f),
			WriteContext::BeforeWhereClause => sql::WHERE.fmt(f),
			WriteContext::InWhereCondition => Ok(()),
		}
	}
}
