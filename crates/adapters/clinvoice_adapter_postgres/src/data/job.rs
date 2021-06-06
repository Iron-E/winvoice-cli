mod deletable;
mod initializable;
mod job_adapter;
mod updatable;

use std::path::PathBuf;

clinvoice_adapter::Adapt!(Job => PostgresJob);

impl PostgresJob<'_, '_>
{
}
