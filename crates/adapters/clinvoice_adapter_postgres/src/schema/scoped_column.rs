mod display;

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) struct PgScopedColumn<TColumn, TIdent>(pub(crate) TIdent, pub(crate) TColumn);
