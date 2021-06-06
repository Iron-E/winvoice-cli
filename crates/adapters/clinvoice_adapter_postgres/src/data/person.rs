mod deletable;
mod initializable;
mod person_adapter;
mod updatable;

use std::path::PathBuf;

clinvoice_adapter::Adapt!(Person => PostgresPerson);

impl PostgresPerson<'_, '_>
{
}
