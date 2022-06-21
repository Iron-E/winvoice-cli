mod display;

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct As<TAlias, TColumn>(pub TColumn, pub TAlias);
