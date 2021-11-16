use super::WriteContext;

impl From<bool> for WriteContext
{
	fn from(b: bool) -> Self
	{
		if b
		{
			WriteContext::AfterClause
		}
		else
		{
			WriteContext::BeforeClause
		}
	}
}
