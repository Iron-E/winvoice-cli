use std::collections::HashMap;

use clinvoice_adapter::schema::{columns::ContactColumns, ContactInfoAdapter};
use clinvoice_match::{MatchContact, MatchSet};
use clinvoice_schema::{Contact, ContactKind, Id};
use futures::TryStreamExt;
use sqlx::{Executor, PgPool, Postgres, QueryBuilder, Result, Row};

use super::PgContactInfo;
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
				(address_id, email, export, label, organization_id, phone) ",
		)
		.push_values(contact_info.iter(), |mut q, (export, kind, label)| {
			q.push_bind(kind.address().map(|a| a.id))
				.push_bind(kind.email())
				.push_bind(export)
				.push_bind(label)
				.push_bind(organization_id)
				.push_bind(kind.phone());
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
		match_condition: &MatchSet<MatchContact>,
	) -> Result<HashMap<Id, Vec<Contact>>>
	{
		const COLUMNS: ContactColumns<&'static str> = ContactColumns::default();

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
		write_where_clause::write_match_contact_set(
			connection,
			Default::default(),
			"C",
			match_condition,
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
					.or_insert_with(Vec::new);
				if let Some(contact) = PgContactInfo::row_to_view(connection, COLUMNS, &row).await?
				{
					entry.push(contact);
				}

				Ok(map)
			})
			.await
	}
}
