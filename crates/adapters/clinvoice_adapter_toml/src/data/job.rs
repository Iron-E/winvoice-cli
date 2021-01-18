mod deletable;
mod into_organization_result;
mod job_adapter;
mod updatable;

const PATH: &str = "Jobs";

clinvoice_adapter::Newtype!(Job => TomlJob);
