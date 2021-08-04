/// # Summary
///
/// Creates a type alias for some [`Error`](std::error::Error) type.
///
/// # Examples
///
/// See any of the `clinvoice` custom [`Error`] types.
#[macro_export]
macro_rules! AliasResult {
	() => {
		clinvoice_error::AliasResult!(Error);
	};

	($error:ident) => {
		pub type Result<T> = std::result::Result<T, $error>;
	};
}
