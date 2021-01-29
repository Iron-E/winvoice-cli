use clinvoice_adapter::{Adapters, Store};
use clinvoice_data::{Id, UUID_NAMESPACE};
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
pub fn unique_id(store_dir: &Path) -> Result<Id, io::Error>
{
	'gen: loop
	{
		let id = Id::new_v5(&UUID_NAMESPACE, Id::new_v4().as_bytes());

		for node in fs::read_dir(store_dir)?
		{
			let node_path = node?.path();
			if match node_path.file_stem()
			{
				Some(stem) => stem.to_string_lossy(),
				None => continue,
			} == id.to_string()
			{ continue 'gen; }
		}

		return Ok(id);
	}
}

#[cfg(test)]
mod tests
{
	use super::{fs, io};
	use std::path::PathBuf;

	#[test]
	fn test_unique_id() -> Result<(), io::Error>
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

			for _ in 0..100
			{
				let id = super::unique_id(&test_path).unwrap();

				// Creating the next file worked.
				assert!(fs::write(&test_path.join(id.to_string()), "TEST").is_ok());
			}
		});
	}
}
