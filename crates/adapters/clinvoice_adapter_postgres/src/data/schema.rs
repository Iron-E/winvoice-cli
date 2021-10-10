mod initializable;

use sqlx::{Error, Executor, Postgres, Result, Transaction};

/// # Summary
///
/// An empty struct which implements [`Initializable`](clinvoice_adapter::data::Initializable) so
/// that the Postgres database can have all of the necessary tables set up if this is the first run
/// of the program.
pub struct PostgresSchema;

impl PostgresSchema
{
	/// # Summary
	///
	/// Initialize the database for a given [`Store`].
	async fn init_locations(connection: impl Executor<'_, Database = Postgres> + Send) -> Result<()>
	{
		sqlx::query!(
			"CREATE TABLE IF NOT EXISTS locations
			(
				id bigint GENERATED ALWAYS AS IDENTITY,
				outer_id bigint CHECK (id <> outer_id),
				name text,

				PRIMARY KEY(id),
				CONSTRAINT locations_outer_id_fk FOREIGN KEY(outer_id) REFERENCES locations(id)
			);"
		).execute(connection).await.and(Ok(()))
	}

	/// # Summary
	/// Initialize the database for a given [`Store`].
	async fn init_people(connection: impl Executor<'_, Database = Postgres> + Send) -> Result<()>
	{
		sqlx::query!(
			"CREATE TABLE IF NOT EXISTS people
			(
				id bigint GENERATED ALWAYS AS IDENTITY,
				name text,

				PRIMARY KEY(id)
			);"
		).execute(connection).await.and(Ok(()))
	}

	/// # Summary
	///
	/// Initialize the database for a given [`Store`].
	async fn init_organizations(connection: impl Executor<'_, Database = Postgres> + Send) -> Result<()>
	{
		sqlx::query!(
			"CREATE TABLE IF NOT EXISTS organizations
			(
				id bigint GENERATED ALWAYS AS IDENTITY,
				location_id bigint NOT NULL,
				name text,

				PRIMARY KEY(id),
				CONSTRAINT organizations_location_id_fk
					FOREIGN KEY(location_id) REFERENCES locations(id)
			);"
		).execute(connection).await.and(Ok(()))
	}

	/// # Summary
	///
	/// Initialize the database for a given [`Store`].
	async fn init_employee_status(connection: impl Executor<'_, Database = Postgres> + Send) -> Result<()>
	{
		sqlx::query!(
			"CREATE TYPE employee_status AS ENUM ('employed', 'not_employed', 'representative');"
		).execute(connection).await.and(Ok(()))
	}

	/// # Summary
	/// Initialize the database for a given [`Store`].
	async fn init_contact_info(connection: impl Executor<'_, Database = Postgres> + Send) -> Result<()>
	{
		sqlx::query!(
			"CREATE TABLE IF NOT EXISTS contact_information
			(
				employee_id bigint NOT NULL,
				export bool NOT NULL,
				name text NOT NULL,

				location_id bigint,
				email text,
				phone text,

				PRIMARY KEY(employee_id, name),
				CONSTRAINT contact_information_employee_id_fk FOREIGN KEY(employee_id) REFERENCES employees(id),
				CONSTRAINT contact_information_location_id_fk FOREIGN KEY(location_id) REFERENCES locations(id),
				CONSTRAINT contact_information_variant_check CHECK
				(
					location_id IS NULL AND
					(
						(email IS NOT NULL AND phone IS NULL) OR -- Contact::Email
						(email IS NULL AND phone IS NOT NULL) -- Contact::Phone
					)
					OR email IS NULL AND phone IS NULL -- Contact::Address
				)
			);"
		).execute(connection).await.and(Ok(()))
	}

	/// # Summary
	///
	/// Initialize the database for a given [`Store`].
	async fn init_employees(connection: impl Executor<'_, Database = Postgres> + Send) -> Result<()>
	{
		sqlx::query!(
			"CREATE TABLE IF NOT EXISTS employees
			(
				id bigint GENERATED ALWAYS AS IDENTITY,
				organization_id bigint NOT NULL,
				person_id bigint NOT NULL,
				status employee_status,
				title text,

				PRIMARY KEY(organization_id, person_id),
				CONSTRAINT employees_organization_id_fk FOREIGN KEY(organization_id) REFERENCES organizations(id),
				CONSTRAINT employees_person_id_fk FOREIGN KEY(person_id) REFERENCES people(id)
			);"
		).execute(connection).await.and(Ok(()))
	}

	/// # Summary
	///
	/// Initialize the database for a given [`Store`].
	async fn init_invoice(connection: &mut Transaction<'_, Postgres>) -> Result<()>
	{
		sqlx::query!(
			"CREATE TYPE invoice_unsafe AS
			(
				date_issued timestamptz,
				date_paid timestamptz,
				hourly_rate amount_of_currency
			);"
		).execute(&mut *connection).await?;

		sqlx::query!(
			"CREATE DOMAIN invoice AS invoice_unsafe CHECK
			(
				VALUE.hourly_rate IS NOT NULL AND
				(VALUE.date_paid IS NULL OR VALUE.date_issued IS NOT NULL)
			);"
		).execute(&mut *connection).await.and(Ok(()))
	}

	/// # Summary
	///
	/// Initialize the database for a given [`Store`].
	async fn init_jobs(connection: impl Executor<'_, Database = Postgres> + Send) -> Result<()>
	{
		sqlx::query!(
			"CREATE TABLE IF NOT EXISTS jobs
			(
				id bigint GENERATED ALWAYS AS IDENTITY,
				client_id bigint NOT NULL,
				date_close timestamptz,
				date_open timestamptz NOT NULL,
				increment interval,
				invoice invoice NOT NULL,
				notes text,
				objectives text NOT NULL,

				PRIMARY KEY(id),
				CONSTRAINT jobs_client_id_fk FOREIGN KEY(client_id) REFERENCES organizations(id)
			);"
		).execute(connection).await.and(Ok(()))
	}

	/// # Summary
	///
	/// Initialize the database for a given [`Store`]. TODO
	async fn init_expense_category(connection: impl Executor<'_, Database = Postgres> + Send) -> Result<()>
	{
		sqlx::query!(
			"CREATE TYPE expense_category AS ENUM ('food', 'item', 'other', 'service', 'software', 'travel');"
		).execute(connection).await.and(Ok(()))
	}

	/// # Summary
	///
	/// Initialize the database for a given [`Store`]. TODO
	async fn init_expenses(connection: &mut Transaction<'_, Postgres>) -> Result<()>
	{
		sqlx::query!(
			"CREATE TYPE expense_unsafe AS
			(
				category expense_category,
				cost amount_of_currency,
				description text
			);"
		).execute(&mut *connection).await?;

		sqlx::query!(
			"CREATE DOMAIN expense AS expense_unsafe CHECK
			(
				VALUE.category IS NOT NULL AND
				VALUE.cost IS NOT NULL AND
				VALUE.description IS NOT NULL
			);"
		).execute(&mut *connection).await.and(Ok(()))
	}

	/// # Summary
	///
	/// Initialize the database for a given [`Store`]. TODO
	async fn init_timesheets(connection: impl Executor<'_, Database = Postgres> + Send) -> Result<()>
	{
		sqlx::query!(
			"CREATE TABLE IF NOT EXISTS employees
			(
				id bigint GENERATED ALWAYS AS IDENTITY,
				organization_id bigint NOT NULL,
				person_id bigint NOT NULL,
				status employee_status,
				title text,

				PRIMARY KEY(organization_id, person_id),
				CONSTRAINT employees_organization_id_fk FOREIGN KEY(organization_id) REFERENCES organizations(id),
				CONSTRAINT employees_person_id_fk FOREIGN KEY(person_id) REFERENCES people(id)
			);"
		).execute(connection).await.and(Ok(()))
	}
}
