use clinvoice_adapter::{Adapters, Error, Store, Result};

/// # Summary
///
/// Derive the connection URI of some given [`Adapters::Postgres`] [`Store`].
pub(crate) fn get_connection_uri(store: &Store) -> Result<String>
{
	if store.adapter != Adapters::Postgres
	{
		return Err(Error::AdapterMismatch {expected: Adapters::Postgres, actual: store.adapter});
	}

	let username = store.username.as_ref().map(|u| u.as_str());
	let password = username.and(store.password.as_ref().map(|p| p.as_str()));

	Ok(format!(
		"postgresql://{}{}{}{}{}",
		username.unwrap_or_default(),
		if password.is_some() {":"} else {""},
		password.unwrap_or_default(),
		if username.is_some() {"@"} else {""},
		store.path,
	))
}

#[cfg(test)]
mod tests
{
	use super::{Adapters, Store};

	#[test]
	fn get_connection_uri()
	{
		// Assert we can't get a connection URI for a non-postgres database.
		assert!(super::get_connection_uri(&Store
		{
			adapter: Adapters::Bincode,
			password: None,
			path: String::new(),
			username: None,
		}).is_err());

		assert_eq!(
			super::get_connection_uri(&Store
			{
				adapter: Adapters::Postgres,
				password: Some("secret".into()),
				path: "localhost".into(),
				username: Some("user".into()),
			}),
			Ok("postgresql://user:secret@localhost".into()),
		);
	}
}
