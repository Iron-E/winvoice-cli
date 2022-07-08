use super::RestoreResult;

/// Implementors of this trait are enabled to preserve data which is important (e.g. a reference
/// number), by [skipping serialization](https://serde.rs/attr-skip-serializing.html) and then using
/// `#[serde(default)]` to generate a temporary value on [deserialization](serde::Deserialize).
///
/// Then, this trait can be used to [restore](RestorableSerde::try_restore) the important data.
pub trait RestorableSerde
{
	/// Take some aspects of an `original` and restore them from [`Default`]s which were assigned
	/// upon [deserialization](serde::Deserialize).
	///
	/// Will return a [`RestoreError`](super::RestoreError) if the `original` cannot be merged with
	/// this value.
	///
	/// # Example
	///
	/// ```rust
	/// use clinvoice_schema::{Employee, RestorableSerde};
	///
	/// let original = Employee {
	///   id: 0, // NOTE: you normally want to avoid assigning an arbitrary ID like this
	///   name: "Bob".into(),
	///   status: "Employed".into(),
	///   title: "CEO".into(),
	/// };
	///
	/// // Pretend this is deserialized user input…
	/// let mut edited = Employee {
	///   id: 3,
	///   name: "Bob Buildman".into(),
	///   ..original.clone()
	/// };
	///
	/// assert_ne!(edited.id, original.id);
	/// assert_ne!(edited.name, original.name);
	/// assert_eq!(edited.status, original.status);
	/// assert_eq!(edited.title, original.title);
	///
	/// edited.try_restore(&original).unwrap();
	///
	/// assert_eq!(edited.id, original.id);
	/// assert_ne!(edited.name, original.name);
	/// assert_eq!(edited.status, original.status);
	/// assert_eq!(edited.title, original.title);
	/// ```
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
