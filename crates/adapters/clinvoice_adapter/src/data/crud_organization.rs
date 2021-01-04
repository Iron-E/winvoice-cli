use crate::Wrapper;

use clinvoice_data::{Location, Organization};

pub trait CrudOrganization<'name, 'rep_title, WLocation>
	: Into<WLocation>
	+ Wrapper<Organization<'name, 'rep_title>>
where
	WLocation : Wrapper<Location<'name>>,
{

}
