use crate::Wrapper;

use clinvoice_data::Employer;

pub trait CrudEmployer<W> where W : Wrapper<Employer>
{

}
