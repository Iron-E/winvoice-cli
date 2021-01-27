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
	let mut highest = -1;

	for node in fs::read_dir(store_dir)?
	{
		let node_path = node?.path();

		if node_path.is_file()
		{
			if let Some(node_stem) = node_path.file_stem()
			{
				if let Ok(id) = node_stem.to_str().unwrap_or("-1").parse::<Id>()
				{
					if id > highest { highest = id; }
				}
			}
		}
	}

	return Ok(highest + 1);
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
	let temp_path = env::temp_dir().join("clinvoice_adapter_bincode_data");

	assertion(&Store
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
	});

	return Ok(());
}

#[cfg(test)]
mod tests
{
	use super::{fs, io};
	use std::path::PathBuf;

	#[test]
	fn test_next_id() -> Result<(), io::Error>
	{
		return super::test_temp_store(|store|
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

				// Creating the next file worked.
				assert!(fs::write(&test_path.join(format!("{}.toml", i)), "").is_ok());
			}
		});
	}
}
