use crate::Wrapper;

use clinvoice_data::Organization;

pub trait CrudOrganization<'name, 'rep_title, W> where W : Wrapper<Organization<'name, 'rep_title>>
{

}
