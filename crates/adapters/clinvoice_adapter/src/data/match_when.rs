use
{
	core::{cmp::Eq, hash::Hash, iter::Iterator},
	std::collections::HashSet,
};

/// # Summary
///
/// A value in a retrieval operation.
pub enum MatchWhen<'range, T> where T : 'range + Hash
{
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
	EqualTo(T),

	/// # Summary
	///
	/// For some value `v`, match if and only if:
	///
	/// * A set of `v` is made up of elements which are contained in this set.
	/// * This set has one element, and `v` is equivalent.
	HasAll(HashSet<T>),

	/// # Summary
	///
	/// For some value `v`, match if and only if:
	///
	/// * A set of `v`'s type has any value contained in this set.
	/// * `v` is contained within this set.
	HasAny(HashSet<T>),

	/// # Summary
	///
	/// For some value `v`, match if and only if:
	///
	/// * A set of `v`'s type has no values contained in this set.
	/// * `v` is not contained within this set.
	HasNone(HashSet<T>),

	/// # Summary
	///
	/// For some value `v`, match if and only if:
	///
	/// * Passing `v` to this function returns `true`.
	/// * A set of `v`'s type all return `true` when passed to this function.
	///
	/// # Example
	///
	/// ```rust
	/// use clinvoice_adapter::data::MatchWhen;
	///
	/// println!("{}", MatchWhen::InRange(&|v| *v > 0 && *v < 5).is_match(&4));
	/// ```
	InRange(&'range dyn Fn(&T) -> bool),
}

impl<'range, T> MatchWhen<'range, T> where T : 'range + Eq + Hash
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
		return match self
		{
			MatchWhen::Any => true,
			MatchWhen::EqualTo(equal_value) => equal_value == value,
			MatchWhen::HasAll(required_values) => required_values.len() == 1 && required_values.contains(value),
			MatchWhen::HasAny(accepted_values) => accepted_values.contains(value),
			MatchWhen::HasNone(denied_values) => !denied_values.contains(value),
			MatchWhen::InRange(in_range) => in_range(value),
		};
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
	pub fn set_matches(&self, values: &HashSet<T>) -> bool
	{
		return match self
		{
			MatchWhen::Any => true,
			MatchWhen::EqualTo(equal_value) => values.len() == 1 && values.contains(equal_value),
			MatchWhen::HasAll(required_values) => required_values.is_subset(values),
			MatchWhen::HasAny(accepted_values) => !accepted_values.is_disjoint(values),
			MatchWhen::HasNone(denied_values) => denied_values.is_disjoint(values),
			MatchWhen::InRange(in_range) => values.iter().all(|v| in_range(v)),
		};
	}
}

#[cfg(test)]
mod tests
{
	use
	{
		super::{HashSet, MatchWhen},
		std::time::Instant,
	};

	#[test]
	fn test_is_match()
	{
		let test_value = &7;

		let has_all: HashSet<i32> = [*test_value].iter().cloned().collect();
		let has_any: HashSet<i32> = [1, 2, 3, *test_value].iter().cloned().collect();
		let has_none: HashSet<i32> = [1, 2, 3].iter().cloned().collect();

		let not_has_all: HashSet<i32> = [3].iter().cloned().collect();
		let not_has_any = has_none.clone();
		let not_has_none = has_any.clone();

		let start = Instant::now();

		// Test any
		assert!(MatchWhen::Any.is_match(test_value));

		// Test equal
		assert!(MatchWhen::EqualTo(7).is_match(test_value));
		assert!(!MatchWhen::EqualTo(6).is_match(test_value));

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
		assert!(MatchWhen::InRange(&|v| *v > 0 && *v < 8).is_match(test_value));
		assert!(!MatchWhen::InRange(&|v| *v > 0 && *v < 3).is_match(test_value));

		println!("\n>>>>> MatchWhen::is_match {}us <<<<<\n", Instant::now().duration_since(start).as_micros());
	}

	#[test]
	fn test_set_matches()
	{
		let test_set: HashSet<i32> = [4, 7, 17].iter().cloned().collect();
		let test_set_single_element: HashSet<i32> = [4].iter().cloned().collect();

		let has_all: HashSet<i32> = [4].iter().cloned().collect();
		let has_any: HashSet<i32> = [1, 4].iter().cloned().collect();
		let has_none: HashSet<i32> = [1].iter().cloned().collect();
		let in_range = |v: &i32| *v > 0 && *v < 18;

		let not_has_all: HashSet<i32> = [4, 6].iter().cloned().collect();
		let not_has_any = has_none.clone();
		let not_has_none = has_any.clone();
		let not_in_range = |v: &i32| *v > 0 && *v < 3;

		let start = Instant::now();

		// Test any
		assert!(MatchWhen::Any.set_matches(&test_set));
		assert!(MatchWhen::Any.set_matches(&test_set_single_element));

		// Test equal
		assert!(!MatchWhen::EqualTo(4).set_matches(&test_set));
		assert!(MatchWhen::EqualTo(4).set_matches(&test_set_single_element));
		assert!(!MatchWhen::EqualTo(6).set_matches(&test_set));
		assert!(!MatchWhen::EqualTo(6).set_matches(&test_set_single_element));

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
		assert!(MatchWhen::InRange(&in_range).set_matches(&test_set));
		assert!(MatchWhen::InRange(&in_range).set_matches(&test_set_single_element));
		assert!(!MatchWhen::InRange(&not_in_range).set_matches(&test_set));
		assert!(!MatchWhen::InRange(&not_in_range).set_matches(&test_set_single_element));

		println!("\n>>>>> MatchWhen::set_match {}us <<<<<\n", Instant::now().duration_since(start).as_micros());
	}
}
