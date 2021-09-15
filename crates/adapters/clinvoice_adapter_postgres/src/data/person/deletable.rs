use clinvoice_data::Person;

use
{
	super::PostgresPerson,
	crate::data::{Error, Result},

	clinvoice_adapter::data::Deletable,
};

#[async_trait::async_trait]
impl<'a> Deletable for PostgresPerson<'a>
{
	type Error = Error;
	type Entity = Person;
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
