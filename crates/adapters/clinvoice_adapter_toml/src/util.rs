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
	let last_dir_path = match fs::read_dir(store_dir)?.last()
	{
		Some(dir_entry) => dir_entry?.path(),
		None => return Ok(0),
	};

	println!("`last_dir_path`: {:?}", last_dir_path);

	let last_dir_path_stem = match last_dir_path.file_stem()
	{
		Some(stem) => stem,
		None => return Err(io::Error::new(
			io::ErrorKind::InvalidData,
			format!("expected `{:?}` to have a filename.", last_dir_path),
		))?,
	};

	println!("`last_dir_id`: {:?}", last_dir_path_stem.to_str().unwrap().parse::<Id>()?);
	println!("`last_dir_next_id`: {:?}", last_dir_path_stem.to_str().unwrap().parse::<Id>()? + 1);

	return match last_dir_path_stem.to_str()
	{
		Some(name) => Ok(name.parse::<Id>()? + 1),
		None => Err(io::Error::new(
			io::ErrorKind::InvalidInput,
			"last file in `store_dir` has bad file stem."
		))?,
	};
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

#[cfg(test)]
mod tests
{
	use super::fs;
	use std::path::PathBuf;

	#[test]
	fn test_next_id()
	{
		assert!(
			super::test_temp_store(|store|
			{
				let test_path = PathBuf::new().join(store.path).join("test_next_id");

				if test_path.is_dir()
				{
					assert!(fs::remove_dir_all(&test_path).is_ok());
				}

				// Create the `test_path`.
				assert!(super::create_store_dir(&test_path).is_ok());

				for i in 0..100
				{
					// The `next_id` matched `i`.
					assert_eq!(super::next_id(&test_path).ok(), Some(i));

					println!("next file: '{}.toml'", i);

					// Creating the next file worked.
					assert!(fs::write(&test_path.join(format!("{}.toml", i)), "").is_ok());

					println!("next file: ...created.\n\n");
				}
			}).is_ok()
		);
	}
}
