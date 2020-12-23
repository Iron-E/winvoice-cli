use crate::Adapters;

/// # Summary
///
/// Connection which _can_ be made to a DBMS.
///
/// # Remarks
///
/// Is __not__ necessarily active!
pub struct Connection<'db, 'url>
{
	/// # Summary
	///
	/// The adapter to use when resolving the connection.
	pub adapter: Adapters,

	/// # Summary
	///
	/// The name of the database to connect to.
	pub database: &'db str,

	/// # Summary
	///
	/// The URL to connect to the DBMS with.
	pub url: &'url str,
}
