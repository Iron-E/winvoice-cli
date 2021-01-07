use clinvoice_data::{chrono::TimeZone, Invoice};

pub trait CrudInvoice<TZone> : From<Invoice<TZone>> where TZone : TimeZone
{

}
