mod deletable;
mod employee_adapter;
mod initializable;
mod updatable;

use std::path::PathBuf;

clinvoice_adapter::Adapt!(Employee => PostgresEmployee);

impl PostgresEmployee<'_, '_>
{
}
