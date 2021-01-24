use clinvoice_adapter::{Adapters, Store};
use clinvoice_data::Id;
use std::{env, error::Error, fs, io, path::Path};

/// # Summary
///
/// Create some `dir` within `store`.
///
/// # Parameters
///
/// * `store_dir`, the directory in the [`Store`] to create.
///
/// # Returns
///
/// * `true`, if the directory was created.
/// * `false`, if the directory already existed.
/// * An `Error`, if `store_dir` couldn't be created.
pub fn create_store_dir(store_dir: &Path) -> Result<bool, io::Error>
{
	if !store_dir.is_dir()
	{
		fs::create_dir_all(store_dir)?;
		return Ok(true);
	}

	return Ok(false);
}

/// # Summary
///
/// Get the next [`Id`] number for an entity in the given `store_dir`.
///
/// # Parameters
///
/// * `store_dir`, the directory in a
///
/// # Returns
///
/// The next [`Id`] for an entity in `store_dir`.
pub fn next_id(store_dir: &Path) -> Result<Id, Box<dyn Error>>
{
	let filename_conv_error = io::Error::new(
		io::ErrorKind::InvalidInput,
		"last file in `store_dir` has bad file name."
	);

	return Ok(match fs::read_dir(store_dir)?.last()
	{
		Some(node) => node?.file_name().to_str().ok_or(filename_conv_error)?.parse::<Id>()?,
		None => 0,
	});
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
