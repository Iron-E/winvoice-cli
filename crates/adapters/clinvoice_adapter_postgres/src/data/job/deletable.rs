use clinvoice_data::Job;

use
{
	super::PostgresJob,
	crate::data::{Error, Result},

	clinvoice_adapter::data::Deletable,
};

#[async_trait::async_trait]
impl<'a> Deletable for PostgresJob<'a>
{
	type Entity = Job;
	type Error = Error;
	type Pool = &'a sqlx::PgPool;

	async fn delete(cascade: bool, entities: &[Self::Entity], pool: &Self::Pool) -> Result<()>
	{
		todo!()
	}
}

#[cfg(test)]
mod tests
{
	#[tokio::test]
	async fn delete()
	{
	}
}
