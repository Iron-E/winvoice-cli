/// # Summary
///
/// An alias to some integer type which is used for storing IDs of records.
#[cfg(uuid)]
pub type Id = uuid::Uuid;

/// # Summary
///
/// An alias to some integer type which is used for storing IDs of records.
#[cfg(not(uuid))]
pub type Id = i64;
