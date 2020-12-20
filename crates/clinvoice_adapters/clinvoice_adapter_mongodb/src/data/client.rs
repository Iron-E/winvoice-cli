use clinvoice_data::client::Client;

pub mod into_organization;

/// # Summary
///
/// Wrapper around [`Client`] for use with MongoDB.
pub struct MongoClient
(
	Client,
);
