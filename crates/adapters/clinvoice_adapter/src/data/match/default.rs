use
{
	super::Match,
	core::hash::Hash,
	std::{cmp::Ord, fmt::Debug},
};


impl<T> Default for Match<'_, T> where
	T : Clone + Debug + Hash + Ord
{
	fn default() -> Self { Self::Any }
}
