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
/// * See [`ColumnsToSql::typecast`](crate::schema::columns::ContactColumns::typecast)
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TypeCast<TColumn, TCast>(pub TColumn, pub TCast);
