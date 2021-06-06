mod deletable;
mod initializable;
mod organization_adapter;
mod updatable;

use std::path::PathBuf;

clinvoice_adapter::Adapt!(Organization => PostgresOrganization);

impl PostgresOrganization<'_, '_>
{
}
