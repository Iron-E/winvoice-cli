use super::{TypeCast, WithIdentifier};

pub struct LocationColumns<T>
{
	pub id: T,
	pub outer_id: T,
	pub name: T,
}

impl<T> LocationColumns<T>
where
	T: Copy,
{
	/// # Summary
	///
	/// Returns an alternation of [`LocationColumns`] which modifies its fields' [`Display`]
	/// implementation to output `{ident}.{column}`.
	pub fn scoped<TIdent>(&self, ident: TIdent) -> LocationColumns<WithIdentifier<T, TIdent>>
	where
		TIdent: Copy,
	{
		LocationColumns {
			id: WithIdentifier(ident, self.id),
			outer_id: WithIdentifier(ident, self.outer_id),
			name: WithIdentifier(ident, self.name),
		}
	}

	/// # Summary
	///
	/// Returns an alternation of [`LocationColumns`] which modifies its fields' [`Display`]
	/// implementation to output `{column}::{cast}`.
	pub fn typecast<TCast>(&self, cast: TCast) -> LocationColumns<TypeCast<TCast, T>>
	where
		TCast: Copy,
	{
		LocationColumns {
			id: TypeCast(self.id, cast),
			outer_id: TypeCast(self.outer_id, cast),
			name: TypeCast(self.name, cast),
		}
	}
}

impl LocationColumns<&'static str>
{
	pub const fn default() -> Self
	{
		Self {
			id: "id",
			outer_id: "outer_id",
			name: "name",
		}
	}
}
