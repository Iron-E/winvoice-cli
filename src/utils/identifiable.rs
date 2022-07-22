use clinvoice_schema::{Contact, Employee, Expense, Job, Location, Organization, Timesheet};

use crate::fmt;

pub trait Identifiable
{
	fn id(&self) -> String;
}

impl Identifiable for Contact
{
	fn id(&self) -> String
	{
		fmt::quoted(&self.label)
	}
}

macro_rules! impl_using_id {
	($T:ty) => {
		impl Identifiable for $T
		{
			fn id(&self) -> String
			{
				fmt::id_num(self.id)
			}
		}
	};
}

macro_rules! impl_using_id_and_name {
	($T:ty) => {
		impl Identifiable for $T
		{
			fn id(&self) -> String
			{
				format!("{} {}", fmt::id_num(self.id), fmt::quoted(&self.name))
			}
		}
	};
}

impl_using_id!(Expense);
impl_using_id!(Job);
impl_using_id!(Timesheet);
impl_using_id_and_name!(Employee);
impl_using_id_and_name!(Location);
impl_using_id_and_name!(Organization);
