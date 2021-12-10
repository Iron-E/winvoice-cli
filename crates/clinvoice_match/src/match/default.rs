use core::{cmp::Ord, fmt::Debug, hash::Hash};

use super::Match;

impl<T> Default for Match<'_, T>
where
	T: Clone + Debug
{
	fn default() -> Self
	{
		Self::Any
	}
}
