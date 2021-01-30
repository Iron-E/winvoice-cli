use std::{cmp::Eq, collections::HashSet, hash::Hash};

/// # Summary
///
/// A value in a retrieval operation.
pub enum RetrieveWhen<T> where T : Eq + Hash
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
	Equal(T),

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
	/// use clinvoice_adapter::data::RetrieveWhen;
	///
	/// println!("{}", RetrieveWhen::InRange(|v| v > 0 && v < 5).is_match(4));
	/// ```
	InRange(Box<dyn Fn(&T) -> bool>),
}

impl<T> RetrieveWhen<T> where T : Eq + Hash
{
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
	/// * `true`, if the `values` match the passed [`RetrieveWhen`].
	/// * `false`, if the `values` do not match.
	pub fn all_match(&self, values: HashSet<T>) -> bool
	{
		return match self
		{
			RetrieveWhen::Any => true,
			RetrieveWhen::Equal(equal_value) => values.len() == 1 && values.contains(equal_value),
			RetrieveWhen::HasAll(required_values) => required_values.is_subset(&values),
			RetrieveWhen::HasAny(accepted_values) => !accepted_values.is_disjoint(&values),
			RetrieveWhen::HasNone(denied_values) => denied_values.is_disjoint(&values),
			RetrieveWhen::InRange(in_range) => values.iter().all(|v| in_range(v)),
		};
	}

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
	/// * `true`, if the `value` matches the passed [`RetrieveWhen`].
	/// * `false`, if the `value` does not match.
	pub fn is_match(&self, value: T) -> bool
	{
		return match self
		{
			RetrieveWhen::Any => true,
			RetrieveWhen::Equal(equal_value) => equal_value == &value,
			RetrieveWhen::HasAll(required_values) => required_values.len() == 1 && required_values.contains(&value),
			RetrieveWhen::HasAny(accepted_values) => accepted_values.contains(&value),
			RetrieveWhen::HasNone(denied_values) => !denied_values.contains(&value),
			RetrieveWhen::InRange(in_range) => in_range(&value),
		};
	}
}

#[cfg(test)]
mod tests
{
	use super::RetrieveWhen;

	#[test]
	fn test_all_match()
	{
	}

	#[test]
	fn test_is_match()
	{
		let test_value = 7;

		// Test any
		assert!(RetrieveWhen::Any.is_match(test_value));

		// Test equal
		assert!(RetrieveWhen::Equal(6).is_match(test_value));
		assert!(RetrieveWhen::Equal(7).is_match(test_value));
	}
}
