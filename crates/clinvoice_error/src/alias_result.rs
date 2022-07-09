/// Creates a `pub type` named `Result`. See examples below.
///
/// # Examples
///
/// For when you have a type named `Error`:
///
/// ```rust
/// use clinvoice_error::AliasResult;
/// # use pretty_assertions::assert_eq;
///
/// {
///   #[derive(Debug, PartialEq)]
///   struct Error;
///
///   AliasResult!();
///   assert_eq!(Result::Ok(()), std::result::Result::<(), Error>::Ok(()));
/// }
///
/// {
///   #[derive(Debug, PartialEq)]
///   struct SomeError;
///
///   AliasResult!(SomeError);
///   assert_eq!(Result::Ok(()), std::result::Result::<(), SomeError>::Ok(()));
/// }
/// ```
#[macro_export]
macro_rules! AliasResult {
	() => {
		clinvoice_error::AliasResult!(Error);
	};

	($error:ident) => {
		pub type Result<T> = std::result::Result<T, $error>;
	};
}
