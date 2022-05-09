pub(super) mod columns;
mod deletable;
mod employee_adapter;
mod updatable;

use core::fmt::Write;
use std::collections::HashMap;

use clinvoice_schema::{Contact, Id};
use sqlx::{Executor, Postgres, Result};

/// Implementor of the [`EmployeeAdapter`](clinvoice_adapter::schema::EmployeeAdapter) for the
/// Postgres database.
pub struct PgEmployee;

impl PgEmployee
{
	/// # Summary
	///
	/// Write some `conact_info` to a database over `connection`. `id` ties the `contact_info` back
	/// to the original [`Employee`](clinvoice_schema::Employee) object.
	///
	/// If `contact_info.is_empty()`, then nothing will happen.
	async fn create_contact_info(
		connection: impl Executor<'_, Database = Postgres> + Send,
		contact_info: &HashMap<String, Contact>,
		id: Id,
	) -> Result<()>
	{
		if !contact_info.is_empty()
		{
			const INSERT_VALUES_APPROX_LEN: u8 = 39;
			let mut contact_info_values =
				String::with_capacity((INSERT_VALUES_APPROX_LEN as usize) * contact_info.len());

			// NOTE: `i * 6` is the number of values each iteration inserts
			(0..contact_info.len()).map(|i| i * 6).for_each(|i| {
				write!(
					contact_info_values,
					"(${}, ${}, ${}, ${}, ${}, ${}),",
					i + 1,
					i + 2,
					i + 3,
					i + 4,
					i + 5,
					i + 6,
				)
				.unwrap()
			});
			contact_info_values.pop(); // get rid of the trailing `,` since SQL can't handle that :/

			contact_info
				.iter()
				.fold(
					sqlx::query(&format!(
						"INSERT INTO contact_information
						(employee_id, label, export, address_id, email, phone)
					VALUES {contact_info_values};",
					)),
					|mut query, (label, contact)| {
						query = query.bind(id).bind(label);

						match contact
						{
							Contact::Address { location, export } => query
								.bind(export)
								.bind(location.id)
								.bind(None::<String>)
								.bind(None::<String>),
							Contact::Email { email, export } => query
								.bind(export)
								.bind(None::<Id>)
								.bind(email)
								.bind(None::<String>),
							Contact::Phone { phone, export } => query
								.bind(export)
								.bind(None::<Id>)
								.bind(None::<String>)
								.bind(phone),
						}
					},
				)
				.execute(connection)
				.await?;
		}

		Ok(())
	}
}
