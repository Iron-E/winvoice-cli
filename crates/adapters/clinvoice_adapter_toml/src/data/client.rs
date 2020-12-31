use clinvoice_data::Client;

mod into_organization;

/// # Summary
///
/// Wrapper around [`Client`] for use with TomlDB.
pub struct TomlClient
(
	Client,
);
