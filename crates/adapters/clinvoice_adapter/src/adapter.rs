use crate::Connection;

pub trait Adapter
{
	/// # Summary
	///
	/// Initialize the database
	fn init(connection: Connection);
}
