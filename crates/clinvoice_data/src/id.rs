/// # Summary
///
/// An alias to some integer type which is used for storing IDs of records.
#[cfg(any(serde_support_unique_id, unique_id))]
pub type Id = uuid::Uuid;

/// # Summary
///
/// An alias to some integer type which is used for storing IDs of records.
#[cfg(not(any(serde_support_unique_id, unique_id)))]
pub type Id = i64;
