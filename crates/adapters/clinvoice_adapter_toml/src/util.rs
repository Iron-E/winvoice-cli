use clinvoice_adapter::{Adapters, Store};
use std::{env, error::Error, fs, io, path::Path};

/// # Summary
///
/// Create some `dir` within `store`.
///
/// # Parameters
///
/// * `store`, the store to reference location with.
/// * `dir`, the directory name to create.
///
/// # Returns
///
/// * `()`, if the directory was created successfully.
/// * An `Error`, if something went wrong.
pub fn create_store_dir(store: &Store<'_, '_, '_>, dir: &str) -> Result<(), Box<dyn Error>>
{
	if store.adapter != Adapters::TOML
	{
		return Err(Box::new(Adapters::TOML.mismatch(&store.adapter)));
	}

	let store_path = Path::new(store.path);

	if store_path.exists()
	{
		for node_result in store_path.read_dir()?
		{
			let node = node_result?.path();

			if node.is_dir() && node.to_str().ok_or_else(|| io::Error::new(
				io::ErrorKind::InvalidInput,
				"The name of the node could not be read as a string."
			))? == dir
			{
				fs::remove_dir(node)?;
			}
		}
	}
	else
	{
		fs::create_dir_all(store_path)?;
	}

	fs::create_dir(store_path.join(dir))?;

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
pub fn test_temp_store(root: &str, assertion: impl FnOnce(&Store<'_, '_, '_>)) -> Result<(), io::Error>
{
	let temp_path = env::temp_dir().join(root);

	assertion(&Store {
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
