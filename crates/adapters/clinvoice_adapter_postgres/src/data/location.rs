mod deletable;
mod initializable;
mod location_adapter;
mod updatable;

use std::path::PathBuf;

clinvoice_adapter::Adapt!(Location => PostgresLocation);

impl PostgresLocation<'_, '_>
{
}
