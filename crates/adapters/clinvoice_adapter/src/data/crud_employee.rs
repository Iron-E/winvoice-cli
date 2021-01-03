use crate::Wrapper;

use clinvoice_data::Employee;

pub trait CrudEmployee<'addr, 'contact_info, 'email, W> where
	'addr  : 'contact_info,
	'email : 'contact_info,
	 W     :  Wrapper<Employee<'addr, 'contact_info, 'email>>,
{

}
