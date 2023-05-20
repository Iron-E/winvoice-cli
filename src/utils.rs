//! Misc utilities for Winvoice.

mod identifiable;

pub use identifiable::Identifiable;
use winvoice_schema::chrono::{DateTime, Datelike, Local, NaiveDateTime, TimeZone, Timelike, Utc};
#[cfg(test)]
use {
	serde::Serialize,
	serde_yaml as yaml,
	std::{env, fs, path::Path},
};

use crate::fmt;

/// Load the `$DATABASE_URL` from a `.env` file, or an environment variable.
#[cfg(test)]
pub(crate) fn database_url() -> dotenvy::Result<String>
{
	dotenvy::var("DATABASE_URL")
}

/// A temporary YAML file which can be used to store data regarding a specific `test`.
#[cfg(test)]
pub(crate) fn temp_file<T>(test: &str) -> std::path::PathBuf
{
	let mut parent = env::temp_dir();
	parent.push("winvoice-bin");
	parent.push(fmt::type_name::<T>());

	fs::create_dir_all(&parent).unwrap();

	parent.push(test);
	parent.set_extension("yaml");
	parent
}

/// Create a [`DateTime<Utc>`] out of some [`Local`] [`NaiveDateTime`].
pub(crate) fn naive_local_datetime_to_utc(d: NaiveDateTime) -> DateTime<Utc>
{
	Local.ymd(d.year(), d.month(), d.day()).and_hms(d.hour(), d.minute(), d.second()).into()
}

/// Indicate with [`println!`] that a value of type `Actioned` — identified by `id` — has been
/// `action`ed.
pub(super) fn report_action<Actioned>(action: &str, actioned: &Actioned)
where
	Actioned: Identifiable,
{
	println!("{} {} has been {action}", fmt::type_name::<Actioned>(), actioned.id(),);
}

/// [`fs::write`][write] some`t`hing to a given `filepath` as [YAML][yaml].
///
/// # Panics
///
/// If either [`serde_yaml`][yaml] or [`fs::write`][write] return [`Err`].
///
/// [write]: std::fs::write
/// [yaml]: serde_yaml::to_string
#[cfg(test)]
pub(crate) fn write_yaml<T>(filepath: &Path, t: T)
where
	T: Serialize,
{
	fs::write(filepath, yaml::to_string(&t).unwrap()).unwrap()
}
