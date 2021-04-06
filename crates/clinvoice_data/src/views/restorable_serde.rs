use
{
	core::hash::Hash,
	std::collections::HashMap,
};

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
	fn restore(&mut self, original: &Self);
}

impl<K, V> RestorableSerde for HashMap<K, V> where
	K : Eq + Hash,
	V : RestorableSerde,
{
	fn restore(&mut self, original: &Self)
	{
		self.iter_mut().for_each(|(key, value)|
			if let Some(original_value) = original.get(key)
			{
				value.restore(original_value)
			}
		);
	}
}
