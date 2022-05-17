//! # Summary
//!
//! This module implements adapters (and associated adapter types such as
//! [`Deletable`](clinvoice_adapter::Deletable)) for a Postgres filesystem.

mod contact_info;
mod employee;
mod expenses;
mod initializable;
mod interval;
mod job;
mod location;
mod option;
mod organization;
mod person;
mod str;
mod timesheet;
mod timestamptz;
mod typecast;
mod util;
mod write_where_clause;

pub use contact_info::PgContactInfo;
pub use employee::PgEmployee;
pub use expenses::PgExpenses;
pub(crate) use interval::PgInterval;
pub use job::PgJob;
pub use location::PgLocation;
pub(crate) use option::PgOption;
pub use organization::PgOrganization;
pub use person::PgPerson;
use sqlx::{Executor, Postgres, Result, Transaction};
pub use timesheet::PgTimesheet;
pub(crate) use timestamptz::PgTimestampTz;

pub(crate) use self::str::PgStr;

/// # Summary
///
/// An empty struct which implements [`Initializable`](clinvoice_adapter::schema::Initializable) so
/// that the Postgres database can have all of the necessary tables set up if this is the first run
/// of the program.
pub struct PgSchema;

impl PgSchema
{
	/// # Summary
	///
	/// Initialize the database for a given [`Store`].
	async fn init_locations(connection: impl Executor<'_, Database = Postgres> + Send)
		-> Result<()>
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
		.await
		.and(Ok(()))
	}

	/// # Summary
	/// Initialize the database for a given [`Store`].
	async fn init_people(connection: impl Executor<'_, Database = Postgres> + Send) -> Result<()>
	{
		sqlx::query!(
			"CREATE TABLE IF NOT EXISTS people
			(
				id bigint GENERATED ALWAYS AS IDENTITY,
				name text NOT NULL,

				PRIMARY KEY(id)
			);"
		)
		.execute(connection)
		.await
		.and(Ok(()))
	}

	/// # Summary
	///
	/// Initialize the database for a given [`Store`].
	async fn init_organizations(
		connection: impl Executor<'_, Database = Postgres> + Send,
	) -> Result<()>
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
		.await
		.and(Ok(()))
	}

	/// # Summary
	///
	/// Initialize the database for a given [`Store`].
	async fn init_employees(connection: impl Executor<'_, Database = Postgres> + Send)
		-> Result<()>
	{
		sqlx::query!(
			"CREATE TABLE IF NOT EXISTS employees
			(
				id bigint GENERATED ALWAYS AS IDENTITY,
				organization_id bigint NOT NULL,
				person_id bigint NOT NULL,
				status text NOT NULL,
				title text NOT NULL,

				PRIMARY KEY(id),
				CONSTRAINT employees__organization_and_person_ids_uq UNIQUE (organization_id, person_id),
				CONSTRAINT employees__organization_id_fk FOREIGN KEY(organization_id) REFERENCES organizations(id),
				CONSTRAINT employees__person_id_fk FOREIGN KEY(person_id) REFERENCES people(id)
			);"
		)
		.execute(connection)
		.await
		.and(Ok(()))
	}

	/// # Summary
	/// Initialize the database for a given [`Store`].
	async fn init_contact_info(
		connection: impl Executor<'_, Database = Postgres> + Send,
	) -> Result<()>
	{
		sqlx::query!(
			r#"CREATE TABLE IF NOT EXISTS contact_information
			(
				employee_id bigint NOT NULL,
				export bool NOT NULL,
				label text NOT NULL,

				address_id bigint,
				email text,
				phone text CHECK (phone ~ '^[0-9\- ]+$'),

				PRIMARY KEY(employee_id, label),
				CONSTRAINT contact_information__employee_id_fk FOREIGN KEY(employee_id) REFERENCES employees(id),
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
		.await
		.and(Ok(()))
	}

	/// # Summary
	///
	/// Initialize the database for a given [`Store`].
	async fn init_money(connection: &mut Transaction<'_, Postgres>) -> Result<()>
	{
		sqlx::query!(r#"CREATE DOMAIN amount_of_currency AS text CHECK (VALUE ~ '^\d+(\.\d+)?$');"#)
			.execute(connection)
			.await
			.and(Ok(()))
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
		.await
		.and(Ok(()))
	}

	/// # Summary
	///
	/// Initialize the database for a given [`Store`].
	async fn init_timesheets(connection: impl Executor<'_, Database = Postgres> + Send)
		-> Result<()>
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
		.await
		.and(Ok(()))
	}

	/// # Summary
	///
	/// Initialize the database for a given [`Store`].
	async fn init_expenses(connection: &mut Transaction<'_, Postgres>) -> Result<()>
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
		.await
		.and(Ok(()))
	}
}
