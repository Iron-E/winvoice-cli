#![allow(clippy::wrong_self_convention)]

use clinvoice_data::{views::PersonView, Person};
use clinvoice_query as query;
use futures::stream::{MapOk, Stream, TryStreamExt};
use sqlx::{Executor, Result};

use super::{Deletable, Updatable};

#[async_trait::async_trait]
pub trait PersonAdapter:
	Deletable<Entity = Person>
	+ Updatable<Db = <Self as Deletable>::Db, Entity = <Self as Deletable>::Entity>
{
	/// # Summary
	///
	/// Create a new [`Person`] on the database.
	///
	/// # Parameters
	///
	/// See [`Person`].
	///
	/// # Returns
	///
	/// The newly created [`Person`].
	async fn create(
		connection: impl 'async_trait + Executor<'_, Database = <Self as Deletable>::Db>,
		name: String,
	) -> Result<<Self as Deletable>::Entity>;

	/// # Summary
	///
	/// Retrieve some [`PersonView`]s from the database using a [query](query::Person).
	///
	/// # Returns
	///
	/// * An `Error`, if something goes wrong.
	/// * A list of matching [`PersonView`]s.
	fn retrieve<'a, E, S>(connection: E, query: &query::Person) -> S
	where
		E: Executor<'a, Database = <Self as Deletable>::Db>,
		S: Stream<Item = Result<<Self as Deletable>::Entity>>;

	/// # Summary
	///
	/// Retrieve some [`PersonView`]s from the database using a [query](query::Person).
	///
	/// # Returns
	///
	/// * An `Error`, if something goes wrong.
	/// * A list of matching [`PersonView`]s.
	fn retrieve_view<'a, E, S>(
		connection: E,
		query: &query::Person,
	) -> MapOk<S, fn(clinvoice_data::Person) -> PersonView>
	where
		E: Executor<'a, Database = <Self as Deletable>::Db>,
		S: Stream<Item = Result<<Self as Deletable>::Entity>>,
	{
		Self::retrieve::<E, S>(connection, query).map_ok(PersonView::from)
	}
}
