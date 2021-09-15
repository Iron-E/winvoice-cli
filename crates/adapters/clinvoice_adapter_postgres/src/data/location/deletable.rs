use
{
	super::PostgresLocation,
	crate::data::{Error, Result},

	clinvoice_adapter::data::Deletable,
	clinvoice_data::Location,
};

#[async_trait::async_trait]
impl<'a> Deletable for PostgresLocation<'a>
{
	type Entity = Location;
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
