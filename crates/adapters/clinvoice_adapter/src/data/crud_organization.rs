use crate::Wrapper;

use clinvoice_data::Organization;

pub trait CrudOrganization<'name, W> where W : Wrapper<Organization<'name>>
{

}
