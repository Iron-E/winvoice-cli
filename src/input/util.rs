pub mod contact;
pub mod employee;
pub mod expense;
pub mod job;
pub mod location;
mod menu;
pub mod organization;
pub mod person;

/// # Summary
///
/// Transforms some `$result` into a view using a `$query`. Meant to capture `Error`s and report
/// them rather than discarding them; only non-matches are discarded.
///
/// # Returns
///
/// * `Ok(_)` =>
///   * `Some(Ok(_))`, if the `$query` matches.
///   * `Some(Err(_))`, if the `$query` returns an `Error`.
///   * `None` if the `$query` does not match.
/// * `Err(e)` => `Some(Err(_))`
#[macro_export]
macro_rules! filter_map_view {
	($query:ident, $result:ident) => {
		match $result
		{
			Ok(val) => match $query.matches_view(&val)
			{
				Ok(matches) if matches => Some(Ok(val)),
				Err(e) => Some(Err(DataError::from(e).into())),
				_ => None,
			},
			Err(e) => Some(Err(e)),
		}
	};
}
