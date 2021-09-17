mod initializable;

/// # Summary
///
/// An empty struct which implements [`Initializable`](clinvoice_adapter::data::Initializable) so
/// that the Postgres database can have all of the necessary tables set up if this is the first run
/// of the program.
pub struct PostgresSchema;
