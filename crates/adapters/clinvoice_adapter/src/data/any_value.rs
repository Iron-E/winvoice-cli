/// # Summary
///
/// A value in a retrieval operation.
pub enum AnyValue<T>
{
	/// # Summary
	///
	/// Any value may be present.
	Any,

	/// # Summary
	///
	/// A specific value must be present.
	Value(Option<T>),
}
