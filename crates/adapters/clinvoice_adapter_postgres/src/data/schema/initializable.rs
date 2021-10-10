use clinvoice_adapter::data::Initializable;
use sqlx::{Acquire, Error, Postgres, Result};

use super::PostgresSchema;

#[async_trait::async_trait]
impl Initializable for PostgresSchema
{
	type Db = Postgres;
	type Error = Error;

	/// # Summary
	///
	/// Initialize the database for a given [`Store`].
	async fn init(connection: impl 'async_trait + Acquire<'_, Database = Self::Db> + Send) -> Result<()>
	{
		let mut transaction = connection.begin().await?;

		/* clinvoice_data::Location */

		sqlx::query!(
			"CREATE TABLE IF NOT EXISTS locations
			(
				id bigint GENERATED ALWAYS AS IDENTITY,
				outer_id bigint CHECK (id <> outer_id),
				name text,

				PRIMARY KEY(id),
				CONSTRAINT locations_outer_id_fk FOREIGN KEY(outer_id) REFERENCES locations(id)
			);"
		).execute(&mut transaction).await?;

		/* clinvoice_data::Person */

		sqlx::query!(
			"CREATE TABLE IF NOT EXISTS people
			(
				id bigint GENERATED ALWAYS AS IDENTITY,
				name text,

				PRIMARY KEY(id)
			);"
		).execute(&mut transaction).await?;

		/* clinvoice_data::Organization */

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
		).execute(&mut transaction).await?;

		/* clinvoice_data::EmployeeStatus */

		sqlx::query!(
			"CREATE TYPE employee_status AS ENUM ('employed', 'not_employed', 'representative');"
		).execute(&mut transaction).await?;

		/* clinvoice_data::Contact */

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
		).execute(&mut transaction).await?;

		/* clinvoice_data::Employee */

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
		).execute(&mut transaction).await?;

		/* clinvoice_finance::Currency */

		sqlx::query!(
			"CREATE TYPE currency AS ENUM
			(
				'AUD', 'BGN', 'BRL', 'CAD', 'CHF', 'CNY', 'CZK', 'DKK', 'EUR', 'GBP', 'HKD',
				'HRK', 'HUF', 'IDR', 'ILS', 'INR', 'ISK', 'JPY', 'KRW', 'MXN', 'MYR', 'NOK',
				'NZD', 'PHP', 'PLN', 'RON', 'RUB', 'SEK', 'SGD', 'THB', 'TRY', 'USD', 'ZAR'
			);"
		).execute(&mut transaction).await?;

		/* clinvoice_finance::Money */

		sqlx::query!(
			"CREATE TYPE amount_of_currency_unsafe AS
			(
				amount money,
				currency currency
			);"
		).execute(&mut transaction).await?;

		sqlx::query!(
			"CREATE DOMAIN amount_of_currency AS amount_of_currency_unsafe CHECK
			(
				VALUE.amount IS NOT NULL AND
				VALUE.currency IS NOT NULL
			);"
		).execute(&mut transaction).await?;

		/* clinvoice_data::Invoice */

		sqlx::query!(
			"CREATE TYPE invoice_unsafe AS
			(
				date_issued timestamptz,
				date_paid timestamptz,
				hourly_rate amount_of_currency
			);"
		).execute(&mut transaction).await?;

		sqlx::query!(
			"CREATE DOMAIN invoice AS invoice_unsafe CHECK
			(
				VALUE.hourly_rate IS NOT NULL AND
				(VALUE.date_paid IS NULL OR VALUE.date_issued IS NOT NULL)
			);"
		).execute(&mut transaction).await?;

		/* clinvoice_data::Job */

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
		).execute(&mut transaction).await?;

		/* clinvoice_data::ExpenseCategory */

		sqlx::query!(
			"CREATE TYPE expense_category AS ENUM ('food', 'item', 'other', 'service', 'software', 'travel');"
		).execute(&mut transaction).await?;

		/* clinvoice_data::Expense */

		sqlx::query!(
			"CREATE TYPE expense_unsafe AS
			(
				category expense_category,
				cost amount_of_currency,
				description text
			);"
		).execute(&mut transaction).await?;

		sqlx::query!(
			"CREATE DOMAIN expense AS expense_unsafe CHECK
			(
				VALUE.category IS NOT NULL AND
				VALUE.cost IS NOT NULL AND
				VALUE.description IS NOT NULL
			);"
		).execute(&mut transaction).await?;

		/* clinvoice_data::Timesheet */

		sqlx::query!(
			"CREATE TABLE IF NOT EXISTS timesheets
			(
				employee_id bigint NOT NULL,
				job_id bigint NOT NULL,
				expenses expense ARRAY,
				time_begin timestamptz NOT NULL,
				time_end timestamptz,
				work_notes text,

				PRIMARY KEY(employee_id, job_id, time_begin),
				CONSTRAINT timesheets_employee_id_fk FOREIGN KEY(employee_id) REFERENCES employees(id),
				CONSTRAINT timesheets_job_id_fk FOREIGN KEY(job_id) REFERENCES jobs(id)
			);"
		).execute(&mut transaction).await?;

		transaction.commit().await
	}
}
