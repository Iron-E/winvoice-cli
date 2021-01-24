use clinvoice_adapter::{Adapters, Store};
use std::{env, fs, io, path::Path};

/// # Summary
///
/// Create some `dir` within `store`.
///
/// # Parameters
///
/// * `store`, the store to reference location with.
///
/// # Returns
///
/// * `()`, if the directory was created successfully.
/// * An `Error`, if something went wrong.
pub fn create_store_dir(store_dir: &Path) -> Result<(), io::Error>
{
	if !store_dir.is_dir() { fs::create_dir_all(store_dir)?; }

	return Ok(());
}

/// # Summary
///
/// Test some `assertion` using a `root` directory within the OS's [temp dir][fn_temp_dir].
///
/// # Parameters
///
/// * `root`, the directory within the [temp dir][fn_temp_dir] to use.
///     * e.g. "foo" -> "%temp%/foo"
/// * `assertion`, the test to run.
///
/// # Returns
///
/// * Nothing, if the `assertion` passed.
/// * An [`Error`](io::Error), if [temp dir][fn_temp_dir] could not be read.
///
/// # Panics
/// If the `assertion` failed.
///
/// [fn_temp_dir]: std::env::temp_dir
pub fn test_temp_store(assertion: impl FnOnce(&Store<'_, '_, '_>)) -> Result<(), io::Error>
{
	let temp_path = env::temp_dir().join("clinvoice_adapter_toml_data");

	assertion(
		&Store
		{
			adapter: Adapters::TOML,
			password: None,
			path: match temp_path.to_str()
			{
				Some(s) => s,
				None => return Err(io::Error::new(
					io::ErrorKind::InvalidInput,
					"`env::temp_path` did not resolve to a valid path."
				)),
			},
			username: None,
		},
	);

	return Ok(());
}
