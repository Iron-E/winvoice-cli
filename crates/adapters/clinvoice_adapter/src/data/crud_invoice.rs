use crate::Wrapper;

use clinvoice_data::{chrono::TimeZone, Invoice};

pub trait CrudInvoice<TZone, W> where
	TZone : TimeZone,
	W : Wrapper<Invoice<TZone>>,
{

}
