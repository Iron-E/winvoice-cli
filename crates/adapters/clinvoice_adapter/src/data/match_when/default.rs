use
{
	super::MatchWhen,
	core::hash::Hash,
	std::{cmp::Ord, fmt::Debug},
};


impl<T> Default for MatchWhen<'_, T> where
	T : Clone + Debug + Hash + Ord
{
	fn default() -> Self { Self::Any }
}
