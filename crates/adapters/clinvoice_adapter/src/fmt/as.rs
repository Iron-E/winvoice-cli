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
/// * See [`ContactColumns::r#as`](crate::schema::columns::ContactColumns::r#as).
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct As<TIdent, TAs>(pub TIdent, pub TAs);
