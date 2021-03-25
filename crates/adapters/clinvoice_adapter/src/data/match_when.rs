use
{
	core::{cmp::Eq, hash::Hash, iter::Iterator},
	std::{borrow::Cow, collections::HashSet, cmp::Ord, fmt::Debug},
	serde::{Deserialize, Serialize},
};

/// # Summary
///
/// A value in a retrieval operation.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature="serde_support", derive(Deserialize, Serialize))]
#[cfg_attr(feature="serde_support", serde(content="value", tag="condition"))]
pub enum MatchWhen<'element, T> where
	T : Clone + Debug + Hash + Ord
{
	#[cfg_attr(feature="serde_support", serde(bound(deserialize = "T : Deserialize<'de>")))]

	/// # Summary
	///
	/// Always match.
	Any,

	/// # Summary
	///
	/// For some value `v`, match if and only if:
	///
	/// * `v` equals this value.
	/// * A set of `v`'s type has one element, and is equal to `v`.
	EqualTo(Cow<'element, T>),

	/// # Summary
	///
	/// For some value `v`, match if and only if:
	///
	/// * A set of `v` is made up of elements which are contained in this set.
	/// * This set has one element, and `v` is equivalent.
	HasAll(HashSet<Cow<'element, T>>),

	/// # Summary
	///
	/// For some value `v`, match if and only if:
	///
	/// * A set of `v`'s type has any value contained in this set.
	/// * `v` is contained within this set.
	HasAny(HashSet<Cow<'element, T>>),

	/// # Summary
	///
	/// For some value `v`, match if and only if:
	///
	/// * A set of `v`'s type has no values contained in this set.
	/// * `v` is not contained within this set.
	HasNone(HashSet<Cow<'element, T>>),

	/// # Summary
	///
	/// For some value `v`, match if and only if:
	///
	/// * The value of `v` is greater than or equal to the first value.
	/// * The value of `v` is less than the first value.
	///
	/// # Example
	///
	/// ```rust
	/// use clinvoice_adapter::data::MatchWhen;
	/// use std::borrow::Cow;
	///
	/// println!("{}", MatchWhen::InRange(Cow::Borrowed(&3),Cow::Borrowed(&5)).is_match(&4));
	/// ```
	InRange(Cow<'element, T>, Cow<'element, T>),
}

/// # Summary
///
/// Return whether or not some [`MatchWhen::InRange`] is in range.
fn is_in_range<T>(min: &T, max: &T, value: &T) -> bool where T : Ord {
	min <= value && value < max
}

impl<'element, T> MatchWhen<'element, T> where
	T : 'element + Clone + Debug + Hash + Ord
{
	/// # Summary
	///
	/// Determine whether or not a `value` matches.
	///
	/// # Parameters
	///
	/// * `value`, the value to check.
	///
	/// # Returns
	///
	/// * `true`, if the `value` matches the passed [`MatchWhen`].
	/// * `false`, if the `value` does not match.
	pub fn is_match(&self, value: &T) -> bool
	{
		match self
		{
			MatchWhen::Any => true,
			MatchWhen::EqualTo(equal_value) => equal_value.as_ref() == value,
			MatchWhen::HasAll(required_values) => required_values.len() == 1 && required_values.contains(value),
			MatchWhen::HasAny(accepted_values) => accepted_values.contains(value),
			MatchWhen::HasNone(denied_values) => !denied_values.contains(value),
			MatchWhen::InRange(min, max) => is_in_range(min.as_ref(), max.as_ref(), value),
		}
	}

	/// # Summary
	///
	/// Determine whether or not the `values` match.
	///
	/// # Parameters
	///
	/// * `values`, the values to check.
	///
	/// # Returns
	///
	/// * `true`, if the `values` match the passed [`MatchWhen`].
	/// * `false`, if the `values` do not match.
	pub fn set_matches(&self, values: &HashSet<&T>) -> bool
	{
		match self
		{
			MatchWhen::Any => true,
			MatchWhen::EqualTo(equal_value) => values.len() == 1 && values.contains(equal_value.as_ref()),
			MatchWhen::HasAll(required_values) => required_values.iter().map(|v| v.as_ref()).collect::<HashSet<_>>().is_subset(values),
			MatchWhen::HasAny(accepted_values) => !accepted_values.iter().map(|v| v.as_ref()).collect::<HashSet<_>>().is_disjoint(values),
			MatchWhen::HasNone(denied_values) => denied_values.iter().map(|v| v.as_ref()).collect::<HashSet<_>>().is_disjoint(values),
			MatchWhen::InRange(min, max) => values.iter().all(|v| is_in_range(min.as_ref(), max.as_ref(), v)),
		}
	}
}

#[cfg(test)]
mod tests
{
	use
	{
		super::{Cow, HashSet, MatchWhen},
		std::time::Instant,
	};

	#[test]
	fn test_is_match()
	{
		let test_value = &7;

		let has_all: HashSet<_> = vec![Cow::Borrowed(&7)].into_iter().collect();
		let has_any: HashSet<_> = vec![Cow::Borrowed(&1), Cow::Borrowed(&2), Cow::Borrowed(&3), Cow::Borrowed(&7)].into_iter().collect();
		let has_none: HashSet<_> = vec![Cow::Borrowed(&1), Cow::Borrowed(&2), Cow::Borrowed(&3)].into_iter().collect();

		let not_has_all: HashSet<_> = vec![Cow::Borrowed(&3)].into_iter().collect();
		let not_has_any = has_none.clone();
		let not_has_none = has_any.clone();

		let start = Instant::now();

		// Test any
		assert!(MatchWhen::Any.is_match(test_value));

		// Test equal
		assert!(MatchWhen::EqualTo(Cow::Borrowed(&7)).is_match(test_value));
		assert!(!MatchWhen::EqualTo(Cow::Borrowed(&6)).is_match(test_value));

		// Test has all
		assert!(MatchWhen::HasAll(has_all).is_match(test_value));
		assert!(!MatchWhen::HasAll(not_has_all).is_match(test_value));

		// Test has any
		assert!(MatchWhen::HasAny(has_any).is_match(test_value));
		assert!(!MatchWhen::HasAny(not_has_any).is_match(test_value));

		// Test has none
		assert!(MatchWhen::HasNone(has_none).is_match(test_value));
		assert!(!MatchWhen::HasNone(not_has_none).is_match(test_value));

		// Test in range
		assert!(MatchWhen::InRange(Cow::Borrowed(&0), Cow::Borrowed(&8)).is_match(test_value));
		assert!(!MatchWhen::InRange(Cow::Borrowed(&0), Cow::Borrowed(&3)).is_match(test_value));

		println!("\n>>>>> MatchWhen::is_match {}us <<<<<\n", Instant::now().duration_since(start).as_micros() / 11);
	}

	#[test]
	fn test_set_matches()
	{
		let test_set: HashSet<_> = [4, 7, 17].iter().collect();
		let test_set_single_element: HashSet<_> = [4].iter().collect();

		let has_all: HashSet<_> = vec![Cow::Borrowed(&4)].into_iter().collect();
		let has_any: HashSet<_> = vec![Cow::Borrowed(&1), Cow::Borrowed(&4)].into_iter().collect();
		let has_none: HashSet<_> = vec![Cow::Borrowed(&1)].into_iter().collect();
		let in_range = MatchWhen::InRange(Cow::Borrowed(&0), Cow::Borrowed(&18));

		let not_has_all: HashSet<_> = vec![Cow::Borrowed(&4), Cow::Borrowed(&6)].into_iter().collect();
		let not_has_any = has_none.clone();
		let not_has_none = has_any.clone();
		let not_in_range = MatchWhen::InRange(Cow::Borrowed(&0), Cow::Borrowed(&3));

		let start = Instant::now();

		// Test any
		assert!(MatchWhen::Any.set_matches(&test_set));
		assert!(MatchWhen::Any.set_matches(&test_set_single_element));

		// Test equal
		assert!(!MatchWhen::EqualTo(Cow::Borrowed(&4)).set_matches(&test_set));
		assert!(MatchWhen::EqualTo(Cow::Borrowed(&4)).set_matches(&test_set_single_element));
		assert!(!MatchWhen::EqualTo(Cow::Borrowed(&6)).set_matches(&test_set));
		assert!(!MatchWhen::EqualTo(Cow::Borrowed(&6)).set_matches(&test_set_single_element));

		// Test has all
		assert!(MatchWhen::HasAll(has_all.clone()).set_matches(&test_set));
		assert!(MatchWhen::HasAll(has_all).set_matches(&test_set_single_element));
		assert!(!MatchWhen::HasAll(not_has_all.clone()).set_matches(&test_set));
		assert!(!MatchWhen::HasAll(not_has_all).set_matches(&test_set_single_element));

		// Test has any
		assert!(MatchWhen::HasAny(has_any.clone()).set_matches(&test_set));
		assert!(MatchWhen::HasAny(has_any).set_matches(&test_set_single_element));
		assert!(!MatchWhen::HasAny(not_has_any.clone()).set_matches(&test_set));
		assert!(!MatchWhen::HasAny(not_has_any).set_matches(&test_set_single_element));

		// Test has none
		assert!(MatchWhen::HasNone(has_none.clone()).set_matches(&test_set));
		assert!(MatchWhen::HasNone(has_none).set_matches(&test_set_single_element));
		assert!(!MatchWhen::HasNone(not_has_none.clone()).set_matches(&test_set));
		assert!(!MatchWhen::HasNone(not_has_none).set_matches(&test_set_single_element));

		// Test in range
		assert!(in_range.set_matches(&test_set));
		assert!(in_range.set_matches(&test_set_single_element));
		assert!(!not_in_range.set_matches(&test_set));
		assert!(!not_in_range.set_matches(&test_set_single_element));

		println!("\n>>>>> MatchWhen::set_match {}us <<<<<\n", Instant::now().duration_since(start).as_micros() / 22);
	}
}
