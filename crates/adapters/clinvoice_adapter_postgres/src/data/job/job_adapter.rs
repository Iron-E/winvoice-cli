use
{
	super::PostgresJob,
	crate::data::{Error, Result},

	clinvoice_adapter::
	{
		data::{Error as DataError, Initializable, JobAdapter, Updatable},
		Store
	},
	clinvoice_data::
	{
		chrono::{DateTime, Utc},
		Invoice, Job, finance::Money, Organization
	},
	clinvoice_query as query,
};

#[async_trait::async_trait]
impl JobAdapter for PostgresJob<'_, '_>
{
	type Error = Error;

	/// # Summary
	///
	/// Create a new [`Person`] on the active [`Store`](crate::Store).
	///
	/// # Paramters
	///
	/// See [`Job`].
	///
	/// # Returns
	///
	/// The newly created [`Person`].
	async fn create(
		client: Organization,
		date_open: DateTime<Utc>,
		hourly_rate: Money,
		objectives: String,
		store: &Store,
	) -> Result<Job>
	{
		todo!()
	}

	/// # Summary
	///
	/// Retrieve some [`Person`] from the active [`Store`](crate::Store).
	///
	/// # Parameters
	///
	/// See [`Job`].
	///
	/// # Returns
	///
	/// * An `Error`, if something goes wrong.
	/// * A list of matching [`Job`]s.
	async fn retrieve(query: &query::Job, store: &Store) -> Result<Vec<Job>>
	{
		todo!()
	}
}

#[cfg(test)]
mod tests
{
	#[tokio::test]
	async fn create()
	{
	}

	#[tokio::test]
	async fn retrieve()
	{
	}
}
