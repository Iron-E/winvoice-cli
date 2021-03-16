/// # Summary
///
/// Defines procedures to preserve specific values during serialization and deserialization, while
/// [hiding](https://serde.rs/attr-skip-serializing.html) the field from the user's view.
pub trait PreservableSerde
{
	/// # Summary
	///
	/// Take some elements of an `original` and restore them from the defaults which were assigned
	/// upon deserialization.
	fn restore(&mut self, original: &Self);
}
