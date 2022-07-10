use clinvoice_adapter::Initializable;
use futures::TryFutureExt;
use sqlx::{Acquire, Executor, Postgres, Result};

use super::PgSchema;

/// Initialize the `locations` table.
async fn init_locations(connection: impl Executor<'_, Database = Postgres>) -> Result<()>
{
	sqlx::query!(
		"CREATE TABLE IF NOT EXISTS locations
		(
			id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
			outer_id bigint REFERENCES locations(id),
			name text NOT NULL,

			CONSTRAINT locations__not_outside_self CHECK (id <> outer_id)
		);"
	)
	.execute(connection)
	.await?;
	Ok(())
}

/// Initialize `organizations` table.
async fn init_organizations(connection: impl Executor<'_, Database = Postgres>) -> Result<()>
{
	sqlx::query!(
		"CREATE TABLE IF NOT EXISTS organizations
		(
			id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
			location_id bigint NOT NULL REFERENCES locations(id),
			name text NOT NULL
		);"
	)
	.execute(connection)
	.await?;
	Ok(())
}

/// Initialize the `employees` table.
async fn init_employees(connection: impl Executor<'_, Database = Postgres> + Send) -> Result<()>
{
	sqlx::query!(
		"CREATE TABLE IF NOT EXISTS employees
		(
			id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
			name text NOT NULL,
			status text NOT NULL,
			title text NOT NULL
		);"
	)
	.execute(connection)
	.await?;
	Ok(())
}

/// Initialize the `contact_information` table.
async fn init_contact_info(connection: impl Executor<'_, Database = Postgres>) -> Result<()>
{
	sqlx::query!(
		r#"CREATE TABLE IF NOT EXISTS contact_information
		(
			label text NOT NULL PRIMARY KEY,

			address_id bigint REFERENCES locations(id),
			email text CHECK (email ~ '^.*@.*\..*$'),
			other text,
			phone text CHECK (phone ~ '^[0-9\- ]+$'),

			CONSTRAINT contact_information__is_variant CHECK
			(
				( -- ContactKind::Address
					address_id IS NOT null AND
					email IS null AND
					other IS null AND
					phone IS null
				)
				OR
				( -- ContactKind::Email
					address_id IS null AND
					email IS NOT null AND
					other IS null AND
					phone IS null
				)
				OR
				( -- ContactKind::Other
					address_id IS null AND
					email IS null AND
					other IS NOT null AND
					phone IS null
				)
				OR
				( -- ContactKind::Phone
					address_id IS null AND
					email IS null AND
					other IS null AND
					phone IS NOT null
				)
			)
		);"#
	)
	.execute(connection)
	.await?;
	Ok(())
}

/// Initialize the `amount_of_currency` type.
async fn init_money(connection: impl Executor<'_, Database = Postgres>) -> Result<()>
{
	sqlx::query!(r#"CREATE DOMAIN amount_of_currency AS text CHECK (VALUE ~ '^\d+(\.\d+)?$');"#)
		.execute(connection)
		.await?;
	Ok(())
}

/// Initialize the `jobs` table.
async fn init_jobs(connection: impl Executor<'_, Database = Postgres>) -> Result<()>
{
	sqlx::query!(
		"CREATE TABLE IF NOT EXISTS jobs
		(
			id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
			client_id bigint NOT NULL REFERENCES organizations(id),
			date_close timestamptz,
			date_open timestamptz NOT NULL,
			increment interval NOT NULL,
			invoice_date_issued timestamptz,
			invoice_date_paid timestamptz,
			invoice_hourly_rate amount_of_currency NOT NULL,
			notes text NOT NULL,
			objectives text NOT NULL,

			CONSTRAINT jobs__invoice_date CHECK (invoice_date_paid IS null OR invoice_date_issued IS NOT null)
		);"
	)
	.execute(connection)
	.await?;
	Ok(())
}

/// Initialize the `timesheets` table.
async fn init_timesheets(connection: impl Executor<'_, Database = Postgres>) -> Result<()>
{
	sqlx::query!(
		"CREATE TABLE IF NOT EXISTS timesheets
		(
			id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
			employee_id bigint NOT NULL REFERENCES employees(id),
			job_id bigint NOT NULL REFERENCES jobs(id),
			time_begin timestamptz NOT NULL,
			time_end timestamptz,
			work_notes text NOT NULL,

			CONSTRAINT timesheets__employee_job_time_uq UNIQUE (employee_id, job_id, time_begin)
		);"
	)
	.execute(connection)
	.await?;
	Ok(())
}

/// Initialize the `expenses` table.
async fn init_expenses(connection: impl Executor<'_, Database = Postgres>) -> Result<()>
{
	sqlx::query!(
		"CREATE TABLE IF NOT EXISTS expenses
		(
			id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
			timesheet_id bigint NOT NULL REFERENCES timesheets(id) ON DELETE CASCADE,
			category text NOT NULL,
			cost amount_of_currency NOT NULL,
			description text NOT NULL
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
