use core::fmt::Debug;

use super::MatchSet;

impl<T> Default for MatchSet<T>
where
	T: Clone + Debug,
{
	fn default() -> Self
	{
		Self::Any
	}
}
