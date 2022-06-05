mod display;

use core::time::Duration;

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) struct PgInterval(pub(crate) Duration);
