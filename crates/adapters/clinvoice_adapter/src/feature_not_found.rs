use thiserror::Error;

use crate::Adapters;

/// # Summary
///
/// [`Error`](std::error::Error)s referencing [`Store`](crate::Store)s and [`Adapters`].
#[derive(Copy, Clone, Debug, Eq, Error, Hash, Ord, PartialEq, PartialOrd)]
#[error("Using this adapter requires the {0} feature")]
pub struct Error (pub Adapters);

clinvoice_error::AliasResult!();
