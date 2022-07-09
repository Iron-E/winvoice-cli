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
/// * See [`ColumnsToSql::scope`](crate::schema::columns::ContactColumns::scope)
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct WithIdentifier<TIdent, TColumn>(pub TIdent, pub TColumn);
