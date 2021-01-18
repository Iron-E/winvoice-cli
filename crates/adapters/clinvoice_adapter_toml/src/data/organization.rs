mod deletable;
mod into_hashset_employee_result;
mod into_location_result;
mod organization_adapter;
mod updatable;

const PATH: &str = "Organizations";

clinvoice_adapter::Newtype!(Organization => TomlOrganization);
