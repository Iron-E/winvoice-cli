use sqlx::{Connection, PgConnection};

#[cfg(test)]
pub(super) async fn connect() -> PgConnection
{
	let database_url = dotenv::var("DATABASE_URL").unwrap();

	let mut conn = PgConnection::connect(&database_url).await.unwrap();
	sqlx::query!("SET SCHEMA 'pg_temp';")
		.execute(&mut conn)
		.await
		.unwrap();
	conn
}
