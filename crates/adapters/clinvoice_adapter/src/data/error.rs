use thiserror::Error;

/// # Summary
///
/// At least one of some entity is necessary to perform an operation, but none were found.
#[derive(Clone, Debug, Error)]
#[error("No {0} could be selected for this operation, and at least one was required")]
pub struct Error (pub String);

clinvoice_error::AliasResult!();
