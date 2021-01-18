mod deletable;
mod employee_adapter;
mod into_organization_result;
mod into_person_result;
mod updatable;

const PATH: &str = "Employees";

clinvoice_adapter::Newtype!(Employee => TomlEmployee);
