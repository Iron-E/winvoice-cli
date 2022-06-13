use super::MatchRow;

impl<T> Default for MatchRow<T>
where
	T: Default,
{
	fn default() -> Self
	{
		MatchRow::EqualTo(Default::default())
	}
}
