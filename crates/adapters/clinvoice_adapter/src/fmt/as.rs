mod display;

/// Formats an SQL `AS` clause.
///
/// # Warnings
///
/// * `TAs`'s and `TIdent`'s [`to_string`](ToString::to_string) output be non-empty to format
///   correctly.
///
/// # Example
///
/// ```rust
/// use clinvoice_adapter::fmt::As;
/// assert_eq!(As("foo.a", "MyAlias").to_string(), "foo.a AS MyAlias");
/// ```
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct As<TIdent, TAs>(pub TIdent, pub TAs);
