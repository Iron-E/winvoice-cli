use std::collections::HashMap;

use clinvoice_adapter::schema::ContactInfoAdapter;
use clinvoice_match::{MatchContact, MatchSet};
use clinvoice_schema::{Contact, ContactKind, Id};
use futures::TryStreamExt;
use sqlx::{Executor, PgPool, Postgres, QueryBuilder, Result, Row};

use super::{columns::PgContactColumns, PgContactInfo};
use crate::schema::write_where_clause;

#[async_trait::async_trait]
impl ContactInfoAdapter for PgContactInfo
{
	async fn create(
		connection: impl 'async_trait + Executor<'_, Database = Postgres> + Send,
		contact_info: Vec<(bool, ContactKind, String)>,
		organization_id: Id,
	) -> Result<Vec<Contact>>
	{
		if contact_info.is_empty()
		{
			return Ok(Vec::new());
		}

		QueryBuilder::new(
			"INSERT INTO contact_information
				(organization_id, label, export, address_id, email, phone)",
		)
		.push_values(contact_info.iter(), |mut q, (export, kind, label)| {
			q.push_bind(organization_id)
				.push_bind(label)
				.push_bind(export);

			match kind
			{
				ContactKind::Address(location) => q
					.push_bind(location.id)
					.push_bind(None::<String>)
					.push_bind(None::<String>),
				ContactKind::Email(email) => q
					.push_bind(None::<Id>)
					.push_bind(email)
					.push_bind(None::<String>),
				ContactKind::Phone(phone) => q
					.push_bind(None::<Id>)
					.push_bind(None::<String>)
					.push_bind(phone),
			};
		})
		.push(';')
		.build()
		.execute(connection)
		.await?;

		Ok(contact_info
			.into_iter()
			.map(|(export, kind, label)| Contact {
				organization_id,
				export,
				kind,
				label,
			})
			.collect())
	}

	async fn retrieve(
		connection: &PgPool,
		match_condition: MatchSet<MatchContact>,
	) -> Result<HashMap<Id, Vec<Contact>>>
	{
		const COLUMNS: PgContactColumns<&'static str> = PgContactColumns::new();

		let mut query = QueryBuilder::new(
			"SELECT
				C.address_id,
				C.email,
				C.export,
				C.label,
				C.phone,
				O.id as organization_id
			FROM organizations O
			LEFT JOIN contact_information C ON (C.organization_id = O.id)",
		);
		write_where_clause::write_contact_set_where_clause(
			connection,
			Default::default(),
			"C",
			&match_condition,
			&mut query,
		)
		.await?;

		query
			.push(';')
			.build()
			.fetch(connection)
			.try_fold(HashMap::new(), |mut map, row| async move {
				let entry = map
					.entry(row.get::<Id, _>(COLUMNS.organization_id))
					.or_insert_with(|| Vec::with_capacity(1));
				if let Some(contact) = COLUMNS.row_to_view(connection, &row).await?
				{
					entry.push(contact);
				}

				Ok(map)
			})
			.await
	}
}
