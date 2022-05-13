use crate::RestoreResult;

/// # Summary
///
/// Defines procedures to preserve specific values during serialization and deserialization, while
/// [hiding](https://serde.rs/attr-skip-serializing.html) the field from the user's view.
pub trait RestorableSerde
{
	/// # Summary
	///
	/// Take some elements of an `original` and restore them from the defaults which were assigned
	/// upon deserialization.
	fn try_restore(&mut self, original: &Self) -> RestoreResult<()>;
}

impl<T> RestorableSerde for Vec<T>
where
	T: RestorableSerde,
{
	fn try_restore(&mut self, original: &Self) -> RestoreResult<()>
	{
		self
			.iter_mut()
			.zip(original)
			.try_for_each(|(edited, original)| edited.try_restore(original))
	}
}
