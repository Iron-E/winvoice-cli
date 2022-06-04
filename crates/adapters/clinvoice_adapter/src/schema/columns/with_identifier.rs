mod display;

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct WithIdentifier<TColumn, TIdent>(pub TIdent, pub TColumn);
