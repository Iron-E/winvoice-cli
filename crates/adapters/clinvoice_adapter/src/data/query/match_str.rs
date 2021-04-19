mod default;

use
{
	regex::{Regex, Error},
	serde::{Deserialize, Serialize},
};

/// # Summary
///
/// A value in a retrieval operation.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature="serde_support", derive(Deserialize, Serialize))]
#[cfg_attr(feature="serde_support", serde(content="value", tag="condition"))]
pub enum MatchStr<S> where S : AsRef<str>
{
	/// # Summary
	///
	/// Always match.
	Any,

	/// # Summary
	///
	/// Match if and only if this value and some other string are exactly the same.
	///
	/// # Example
	///
	/// ```rust
	/// use clinvoice_adapter::data::MatchStr;
	///
	/// assert!(MatchStr::EqualTo("Foo").matches("Foo"));
	/// assert!(MatchStr::EqualTo("Foo").matches(["Foo", "Bar"]));
	/// ```
	EqualTo(S),

	/// # Summary
	///
	/// Match if and only if:
	///
	/// * Some string contains this value anywhere in its contents.
	/// * A set of strings contains this value anywhere in its contents.
	///
	/// # Example
	///
	/// ```rust
	/// use clinvoice_adapter::data::MatchStr;
	///
	/// assert!(MatchStr::Contains("Foo").matches("Foobar"));
	/// assert!(MatchStr::Contains("Foo").matches("barfoo"));
	/// assert!(MatchStr::Contains("Foo").matches(["bar", "foo"]));
	/// ```
	Contains(S),

	/// # Summary
	///
	/// Match if and only if:
	///
	/// * This regular expression matches some other string.
	/// * This regular expression matches any string in a set of strings.
	///
	/// # Example
	///
	/// ```rust
	/// use clinvoice_adapter::data::MatchStr;
	///
	/// assert!(MatchStr::Regex("^Foo").matches("Foobar"));
	/// assert!(MatchStr::Regex("foo$").matches(["Bar", "foo"]));
	/// ```
	Regex(S),
}

impl<S> MatchStr<S> where S : AsRef<str> + Eq
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
	/// * `true`, if the `value` matches the passed [`MatchStr`].
	/// * `false`, if the `value` does not match.
	pub fn matches(&self, value: &str) -> Result<bool, Error>
	{
		Ok(match self
		{
			Self::Any => true,
			Self::EqualTo(equal_value) => equal_value.as_ref() == value,
			Self::Contains(contained_value) => value.contains(contained_value.as_ref()),
			Self::Regex(expression) => Regex::new(expression.as_ref())?.is_match(value),
		})
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
	/// * `true`, if the `values` match the passed [`MatchStr`].
	/// * `false`, if the `values` do not match.
	pub fn set_matches<'item>(&self, mut values: impl Iterator<Item=&'item str>) -> Result<bool, Error>
	{
		Ok(match self
		{
			Self::Any => true,
			Self::EqualTo(equal_value) =>
			{
				let equal_str = equal_value.as_ref();
				values.any(|v| v.contains(equal_str))
			},
			Self::Contains(contained_value) =>
			{
				let contained_str = contained_value.as_ref();
				values.any(|v| v.contains(contained_str))
			},
			Self::Regex(expression) =>
			{
				let regex = Regex::new(expression.as_ref())?;
				values.any(|v| regex.is_match(v))
			},
		})
	}
}
