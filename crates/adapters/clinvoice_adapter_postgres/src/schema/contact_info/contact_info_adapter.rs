use clinvoice_adapter::{
	fmt::{sql, QueryBuilderExt, TableToSql},
	schema::{columns::ContactColumns, ContactInfoAdapter},
};
use clinvoice_match::MatchContact;
use clinvoice_schema::Contact;
use futures::TryStreamExt;
use sqlx::{Executor, PgPool, Postgres, QueryBuilder, Result};

use super::PgContactInfo;
use crate::schema::write_where_clause;

#[async_trait::async_trait]
impl ContactInfoAdapter for PgContactInfo
{
	async fn create(
		connection: impl 'async_trait + Executor<'_, Database = Postgres> + Send,
		contact_info: impl 'async_trait + Iterator<Item = &Contact> + Send,
	) -> Result<()>
	{
		let mut peekable = contact_info.peekable();
		if peekable.peek().is_some()
		{
			QueryBuilder::new(
				"INSERT INTO contact_information
					(address_id, email, label, other, phone) ",
			)
			.push_values(peekable, |mut q, contact| {
				q.push_bind(contact.kind.address().map(|a| a.id))
					.push_bind(contact.kind.email())
					.push_bind(&contact.label)
					.push_bind(contact.kind.other())
					.push_bind(contact.kind.phone());
			})
			.prepare()
			.execute(connection)
			.await?;
		}

		Ok(())
	}

	async fn retrieve(connection: &PgPool, match_condition: &MatchContact) -> Result<Vec<Contact>>
	{
		const COLUMNS: ContactColumns<&'static str> = ContactColumns::default();

		let mut query = QueryBuilder::new(sql::SELECT);

		query
			.push_columns(&COLUMNS.default_scope())
			.push_default_from::<ContactColumns<char>>();

		write_where_clause::write_match_contact(
			connection,
			Default::default(),
			ContactColumns::<char>::DEFAULT_ALIAS,
			match_condition,
			&mut query,
		)
		.await?;

		query
			.prepare()
			.fetch(connection)
			.and_then(|row| async move { PgContactInfo::row_to_view(connection, COLUMNS, &row).await })
			.try_collect()
			.await
	}
}
