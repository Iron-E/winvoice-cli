use crate::Wrapper;

use clinvoice_data::{chrono::TimeZone, Invoice};

pub trait CrudInvoice<TZone> : Wrapper<Invoice<TZone>> where TZone : TimeZone
{

}
