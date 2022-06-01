use clinvoice_adapter::Initializable;
use futures::TryFutureExt;
use sqlx::{Acquire, Error, Executor, Postgres, Result};

use super::PgSchema;

/// # Summary
///
/// Initialize the database for a given [`Store`].
async fn init_locations(connection: impl Executor<'_, Database = Postgres> + Send) -> Result<()>
{
	sqlx::query!(
		"CREATE TABLE IF NOT EXISTS locations
		(
			id bigint GENERATED ALWAYS AS IDENTITY,
			outer_id bigint,
			name text NOT NULL,

			PRIMARY KEY(id),
			CONSTRAINT locations__not_outside_self CHECK (id <> outer_id),
			CONSTRAINT locations__outer_id_fk FOREIGN KEY(outer_id) REFERENCES locations(id)
		);"
	)
	.execute(connection)
	.await?;
	Ok(())
}

/// # Summary
///
/// Initialize the database for a given [`Store`].
async fn init_organizations(connection: impl Executor<'_, Database = Postgres> + Send)
	-> Result<()>
{
	sqlx::query!(
		"CREATE TABLE IF NOT EXISTS organizations
		(
			id bigint GENERATED ALWAYS AS IDENTITY,
			location_id bigint NOT NULL,
			name text NOT NULL,

			PRIMARY KEY(id),
			CONSTRAINT organizations__location_id_fk
				FOREIGN KEY(location_id) REFERENCES locations(id)
		);"
	)
	.execute(connection)
	.await?;
	Ok(())
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
			name text NOT NULL,
			organization_id bigint NOT NULL,
			status text NOT NULL,
			title text NOT NULL,

			PRIMARY KEY(id),
			CONSTRAINT employees__organization_id_fk FOREIGN KEY(organization_id) REFERENCES organizations(id)
		);"
	)
	.execute(connection)
	.await?;
	Ok(())
}

/// # Summary
/// Initialize the database for a given [`Store`].
async fn init_contact_info(connection: impl Executor<'_, Database = Postgres> + Send)
	-> Result<()>
{
	sqlx::query!(
		r#"CREATE TABLE IF NOT EXISTS contact_information
		(
			organization_id bigint NOT NULL,
			export bool NOT NULL,
			label text NOT NULL,

			address_id bigint,
			email text,
			phone text CHECK (phone ~ '^[0-9\- ]+$'),

			PRIMARY KEY(organization_id, label),
			CONSTRAINT contact_information__organization_id_fk FOREIGN KEY(organization_id) REFERENCES organizations(id),
			CONSTRAINT contact_information__address_id_fk FOREIGN KEY(address_id) REFERENCES locations(id),
			CONSTRAINT contact_information__is_variant CHECK
			(
				address_id IS NULL AND
				(
					(email IS NOT NULL AND phone IS NULL) OR -- ContactKind::Email
					(email IS NULL AND phone IS NOT NULL) -- ContactKind::Phone
				)
				OR email IS NULL AND phone IS NULL -- ContactKind::Address
			)
		);"#
	)
	.execute(connection)
	.await?;
	Ok(())
}

/// # Summary
///
/// Initialize the database for a given [`Store`].
async fn init_money(connection: impl Executor<'_, Database = Postgres> + Send) -> Result<()>
{
	sqlx::query!(r#"CREATE DOMAIN amount_of_currency AS text CHECK (VALUE ~ '^\d+(\.\d+)?$');"#)
		.execute(connection)
		.await?;
	Ok(())
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
			increment interval NOT NULL,
			invoice_date_issued timestamptz,
			invoice_date_paid timestamptz,
			invoice_hourly_rate amount_of_currency NOT NULL,
			notes text NOT NULL,
			objectives text NOT NULL,

			PRIMARY KEY(id),
			CONSTRAINT jobs__client_id_fk FOREIGN KEY(client_id) REFERENCES organizations(id),
			CONSTRAINT jobs__invoice_date CHECK (invoice_date_paid IS NULL OR invoice_date_issued IS NOT NULL)
		);"
	)
	.execute(connection)
	.await?;
	Ok(())
}

/// # Summary
///
/// Initialize the database for a given [`Store`].
async fn init_timesheets(connection: impl Executor<'_, Database = Postgres> + Send) -> Result<()>
{
	sqlx::query!(
		"CREATE TABLE IF NOT EXISTS timesheets
		(
			id bigint GENERATED ALWAYS AS IDENTITY,
			employee_id bigint NOT NULL,
			job_id bigint NOT NULL,
			time_begin timestamptz NOT NULL,
			time_end timestamptz,
			work_notes text NOT NULL,

			PRIMARY KEY(id),
			CONSTRAINT timesheets__employee_id_fk FOREIGN KEY(employee_id) REFERENCES employees(id),
			CONSTRAINT timesheets__employee_job_time_uq UNIQUE (employee_id, job_id, time_begin),
			CONSTRAINT timesheets__job_id_fk FOREIGN KEY(job_id) REFERENCES jobs(id)
		);"
	)
	.execute(connection)
	.await?;
	Ok(())
}

/// # Summary
///
/// Initialize the database for a given [`Store`].
async fn init_expenses(connection: impl Executor<'_, Database = Postgres> + Send) -> Result<()>
{
	sqlx::query!(
		"CREATE TABLE IF NOT EXISTS expenses
		(
			id bigint GENERATED ALWAYS AS IDENTITY,
			timesheet_id bigint NOT NULL,
			category text NOT NULL,
			cost amount_of_currency NOT NULL,
			description text NOT NULL,

			PRIMARY KEY(id),
			CONSTRAINT expenses__timesheet_id_fk FOREIGN KEY(timesheet_id) REFERENCES timesheets(id)
		);"
	)
	.execute(connection)
	.await?;
	Ok(())
}

#[async_trait::async_trait]
impl Initializable for PgSchema
{
	type Db = Postgres;
	type Error = Error;

	/// # Summary
	///
	/// Initialize the database for a given [`Store`].
	async fn init(
		connection: impl 'async_trait + Acquire<'_, Database = Self::Db> + Send,
	) -> Result<()>
	{
		connection
			.begin()
			.and_then(|mut transaction| async move {
				init_locations(&mut transaction).await?;
				init_organizations(&mut transaction).await?;
				init_contact_info(&mut transaction).await?;
				init_employees(&mut transaction).await?;
				init_money(&mut transaction).await?;
				init_jobs(&mut transaction).await?;
				init_timesheets(&mut transaction).await?;
				init_expenses(&mut transaction).await?;

				transaction.commit().await
			})
			.await
	}
}
