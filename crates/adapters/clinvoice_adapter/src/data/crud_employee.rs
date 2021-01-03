use crate::Wrapper;

use clinvoice_data::Employee;

pub trait CrudEmployee<W> where W : Wrapper<Employee>
{

}
