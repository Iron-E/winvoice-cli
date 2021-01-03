use crate::Wrapper;

use clinvoice_data::Person;

pub trait CrudPerson<'name, W> where W : Wrapper<Person<'name>>
{

}
