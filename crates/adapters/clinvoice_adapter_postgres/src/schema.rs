//! # Summary
//!
//! This module implements adapters (and associated adapter types such as
//! [`Deletable`](clinvoice_adapter::Deletable)) for a Postgres filesystem.

mod contact_info;
mod employee;
mod expenses;
mod initializable;
mod interval;
mod job;
mod location;
mod option;
mod organization;
mod str;
mod timesheet;
mod timestamptz;
mod typecast;
mod util;
mod write_where_clause;

pub use contact_info::PgContactInfo;
pub use employee::PgEmployee;
pub use expenses::PgExpenses;
pub(crate) use interval::PgInterval;
pub use job::PgJob;
pub use location::PgLocation;
pub(crate) use option::PgOption;
pub use organization::PgOrganization;
pub use timesheet::PgTimesheet;
pub(crate) use timestamptz::PgTimestampTz;

pub(crate) use self::str::PgStr;

/// # Summary
///
/// An empty struct which implements [`Initializable`](clinvoice_adapter::schema::Initializable) so
/// that the Postgres database can have all of the necessary tables set up if this is the first run
/// of the program.
pub struct PgSchema;
