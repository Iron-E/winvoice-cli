mod display;

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) struct PgTypeCast<TCast, TColumn>(pub(crate) TColumn, pub(crate) TCast);
