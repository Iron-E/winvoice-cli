use clinvoice_adapter::{Adapters, Store};
use std::{fs, error::Error, io::{Error as IOError, ErrorKind}, path::Path};

/// # Summary
///
/// Initialize the postgresql database on [`Store`].
pub fn create_store_dir<'err, 'pass, 'path, 'user>(store: Store<'pass, 'path, 'user>, dir: &str) -> Result<(), Box<dyn Error>>
{
	if store.adapter != Adapters::TOML
	{
		return Err(Box::new(Adapters::TOML.mismatch(store.adapter)));
	}

	let store_path = Path::new(store.path);

	if store_path.exists()
	{
		if match store_path.read_dir() { Ok(r) => r.count(), Err(e) => return Err(Box::new(e)) } > 0
		{
			return Err(Box::new(IOError::new(
				ErrorKind::AlreadyExists, format!("The specified path, {}, is already in use.", store.path)
			)));
		}
	}
	else if let Err(e) = fs::create_dir_all(store_path)
	{
		return Err(Box::new(e));
	}

	return match fs::create_dir(store_path.join(Path::new(dir)))
	{
		Ok(_) => Ok(()),
		Err(e) => Err(Box::new(e)),
	};
}
