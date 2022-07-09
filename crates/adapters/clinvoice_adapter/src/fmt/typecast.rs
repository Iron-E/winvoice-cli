mod display;

/// Formats an SQL `CAST` expression.
///
/// # Warnings
///
/// * `TCast`'s and `TColumn`'s [`to_string`](ToString::to_string) output be non-empty to format
///   correctly.
///
/// # Example
///
/// ```rust
/// use clinvoice_adapter::fmt::TypeCast;
/// # use pretty_assertions::assert_eq;
///
/// assert_eq!(TypeCast("foo.a", "numeric").to_string(), " CAST (foo.a AS numeric)");
/// ```
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TypeCast<TColumn, TCast>(pub TColumn, pub TCast);
