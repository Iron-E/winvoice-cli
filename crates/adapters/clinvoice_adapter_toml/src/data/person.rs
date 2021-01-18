mod deletable;
mod person_adapter;
mod updatable;

const PATH: &str = "People";

clinvoice_adapter::Newtype!(Person => TomlPerson);
