use clinvoice_data::Employee;

use
{
	super::PostgresEmployee,
	crate::data::{Error, Result},

	clinvoice_adapter::data::Deletable,
};

#[async_trait::async_trait]
impl<'pool> Deletable for PostgresEmployee<'pool>
{
	type Entity = Employee;
	type Error = Error;
	type Pool = &'pool sqlx::PgPool;

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
