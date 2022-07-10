mod display;

/// Formats a dot-access of `TColumn`.
///
/// # Warnings
///
/// * `TColumn`'s and `TIdent`'s [`to_string`](ToString::to_string) output be non-empty to format
///   correctly.
///
/// # Example
///
/// ```rust
/// use clinvoice_adapter::fmt::WithIdentifier;
/// # use pretty_assertions::assert_eq;
///
/// assert_eq!(WithIdentifier("foo", "a").to_string(), "foo.a");
/// ```
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct WithIdentifier<TIdent, TColumn>(pub TIdent, pub TColumn);
