/// # Summary
///
/// `Wrapper` defines a clear communication line for `Wrapped` items.
pub trait Wrapper<Wrapped>
{
	/// # Summary
	///
	/// Unroll the `Wrapped` item.
	///
	/// # Returns
	///
	/// The item which this wrapper is wrapping.
	fn unroll(self) -> Wrapped;
}
