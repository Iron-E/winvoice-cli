mod deletable;
mod display;
mod location_adapter;
mod updatable;

const PATH: &str = "Locations";

clinvoice_adapter::Newtype!(Location => TomlLocation);
