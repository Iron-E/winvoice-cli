use core::fmt::Display;

use sqlx::{Database, QueryBuilder};

pub trait QueryBuilderExt
{
	/// # Summary
	///
	/// Push `left = right`.
	fn push_equal<TLeft, TRight>(&mut self, left: TLeft, right: TRight) -> &mut Self
	where
		TLeft: Display,
		TRight: Display;
}

impl<Db> QueryBuilderExt for QueryBuilder<'_, Db>
where
	Db: Database,
{
	fn push_equal<TLeft, TRight>(&mut self, left: TLeft, right: TRight) -> &mut Self
	where
		TLeft: Display,
		TRight: Display,
	{
		self.separated('=').push(left).push(right);
		self
	}
}
