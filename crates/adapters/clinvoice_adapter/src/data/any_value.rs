use std::{cmp::Eq, collections::HashSet, hash::Hash};

/// # Summary
///
/// A value in a retrieval operation.
pub enum AnyValue<T> where T : Eq + Hash
{
	/// # Summary
	///
	/// Match if and only if:
	///
	/// 1. This set is a superset of some other compared data.
	Accept(HashSet<T>),

	/// # Summary
	///
	/// A combination of [`Accept`](AnyValue::Accept) and [`All`](AnyValue::All).
	///
	/// Matches if and only if:
	///
	/// 1. This set is a superset of some other compared data.
	/// 2. All values in this set are present in some other compared data.
	AcceptAll(HashSet<T>),

	/// # Summary
	///
	/// Match if and only if:
	///
	/// 1. This set is a subset of some other compared data.
	All(HashSet<T>),

	/// # Summary
	///
	/// Always match.
	Any,

	/// # Summary
	///
	/// Match if and only if:
	///
	/// 1. The compared data does not contain any of the values from this set.
	Except(HashSet<T>),

	/// # Summary
	///
	/// Match if and only if:
	///
	/// 1. Some `z` is greater than `Range(x, y).0`; and
	/// 2. Some `z` is less than `Range(x, y).1`.
	///
	/// # Example
	///
	/// ```rust
	/// use clinvoice_adapter::AnyValue;
	///
	/// println!("{}", AnyValue::Range(|v| v > 0 && v < 5).is_match(4));
	/// ```
	Range(Box<dyn Fn(&T) -> bool>),

	/// # Summary
	///
	/// Match if and only if:
	///
	/// 1. The compared data is this value.
	Value(T),
}

impl<T> AnyValue<T> where T : Eq + Hash
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
	/// * `true`, if the `values` match the passed [`AnyValue`].
	/// * `false`, if the `values` do not match.
	pub fn all_match(&self, values: HashSet<T>) -> bool
	{
		return match self
		{
			AnyValue::Any => true,

			AnyValue::Accept(accepted_values) => accepted_values.is_superset(&values),

			AnyValue::AcceptAll(all_accepted_values) =>
				all_accepted_values.is_superset(&values) &&
				all_accepted_values.is_subset(&values),

			AnyValue::All(all_values) => all_values.is_subset(&values),

			AnyValue::Except(excepted) => excepted.is_disjoint(&values),

			AnyValue::Range(in_range) => values.iter().all(|v| in_range(v)),

			AnyValue::Value(value) => values.iter().all(|v| v == value),
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
	/// * `true`, if the `value` matches the passed [`AnyValue`].
	/// * `false`, if the `value` does not match.
	pub fn is_match(&self, value: T) -> bool
	{
		return match self
		{
			AnyValue::Any => true,

			AnyValue::Accept(accepted_values) => accepted_values.contains(&value),

			AnyValue::AcceptAll(all_accepted_values) =>
				all_accepted_values.len() == 1 &&
				all_accepted_values.contains(&value),

			AnyValue::All(all_values) => all_values.len() == 1 && all_values.contains(&value),

			AnyValue::Except(excepted) => !excepted.contains(&value),

			AnyValue::Range(in_range) => in_range(&value),

			AnyValue::Value(v) => v == &value,
		};
	}
}
