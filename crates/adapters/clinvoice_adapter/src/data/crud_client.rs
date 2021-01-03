use crate::Wrapper;

use clinvoice_data::Client;

pub trait CrudClient<W> where W : Wrapper<Client>
{

}
