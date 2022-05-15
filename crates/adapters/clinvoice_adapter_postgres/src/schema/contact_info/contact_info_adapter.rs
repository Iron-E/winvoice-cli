use core::fmt::Write;

use clinvoice_adapter::schema::ContactInfoAdapter;
use clinvoice_match::{MatchContact, MatchSet};
use clinvoice_schema::{Contact, ContactKind, Id};
use futures::stream::TryStreamExt;
use sqlx::{Executor, PgPool, Postgres, Result};

use super::{columns::PgContactColumns, PgContactInfo};
use crate::schema::write_where_clause;

#[async_trait::async_trait]
impl ContactInfoAdapter for PgContactInfo
{
	async fn create(
		connection: impl 'async_trait + Executor<'_, Database = Postgres> + Send,
		contact_info: &[(bool, ContactKind, String)],
		employee_id: Id,
	) -> Result<Vec<Contact>>
	{
		if contact_info.is_empty()
		{
			return Ok(Vec::new());
		}

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
				|mut query, (export, kind, label)| {
					query = query.bind(employee_id).bind(label).bind(export);

					match kind
					{
						ContactKind::Address(location) => query
							.bind(location.id)
							.bind(None::<String>)
							.bind(None::<String>),
						ContactKind::Email(email) =>
						{
							query.bind(None::<Id>).bind(email).bind(None::<String>)
						},
						ContactKind::Phone(phone) =>
						{
							query.bind(None::<Id>).bind(None::<String>).bind(phone)
						},
					}
				},
			)
			.execute(connection)
			.await?;

		Ok(contact_info
			.iter()
			.cloned()
			.map(|(export, kind, label)| Contact {
				employee_id,
				export,
				kind,
				label,
			})
			.collect())
	}

	async fn retrieve(
		connection: &PgPool,
		match_condition: MatchSet<MatchContact>,
	) -> Result<Vec<Contact>>
	{
		let mut query = String::from("SELECT * FROM contact_info C");
		write_where_clause::write_contact_set_where_clause(
			connection,
			Default::default(),
			"C",
			&match_condition,
			&mut query,
		)
		.await?;
		query.push(';');

		const COLUMNS: PgContactColumns<'static> = PgContactColumns {
			employee_id: "employee_id",
			export: "export",
			label: "label",
			address_id: "address_id",
			email: "email",
			phone: "phone",
		};

		sqlx::query(&query)
			.fetch(connection)
			.and_then(|row| async move { COLUMNS.row_to_view(connection, &row).await })
			.try_collect()
			.await
	}
}