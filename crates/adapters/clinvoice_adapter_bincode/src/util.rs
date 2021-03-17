use
{
	clinvoice_adapter::Store,
	clinvoice_data::{Id, UUID_NAMESPACE},
	std::{fs, io, iter::FilterMap, path::{Path, PathBuf}},
};

#[cfg(test)]
use
{
	clinvoice_adapter::Adapters,
	std::env,
};

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
pub fn create_store_dir(store_dir: &Path) -> io::Result<bool>
{
	if !store_dir.is_dir()
	{
		fs::create_dir_all(store_dir)?;
		return Ok(true);
	}

	Ok(false)
}

/// # Summary
///
/// Expand the `store`'s specified path and join the provided `subdir`.
pub fn expand_store_path(store: &Store) -> PathBuf
{
	match shellexpand::full(&store.path)
	{
		Ok(p) => PathBuf::from(p.as_ref()),
		_ => PathBuf::from(&store.path),
	}
}

/// # Summary
///
/// Return a [`FilterMap`] iterating over all valid [`File`](fs::File)s in some `path`.
///
/// # Errors
///
/// Will error whenever [`fs::read_dir`] does.
pub fn read_files<P>(path: P) -> io::Result<FilterMap<fs::ReadDir, impl FnMut(io::Result<fs::DirEntry>) -> Option<PathBuf>>> where
	P : AsRef<Path>,
{
	Ok(fs::read_dir(path)?.filter_map(
		|node| match node
		{
			Ok(n) if n.path().is_file() => Some(n.path()),
			_ => None,
		}
	))
}

/// # Summary
///
/// Test some `assertion` using a `root` directory within the OS's [temp dir][fn_temp_dir].
///
/// # Remarks
///
/// `root` is joined with the [temp dir][fn_temp_dir] (e.g. "foo" -> "%temp%/foo").
///
/// # Returns
///
/// * Nothing, if the `assertion` passed.
/// * An [`Error`](io::Error), if [temp dir][fn_temp_dir] could not be read.
///
/// [fn_temp_dir]: std::env::temp_dir
#[cfg(test)]
pub fn test_temp_store(assertion: impl FnOnce(&Store))
{
	let temp_path = env::temp_dir().join("clinvoice_adapter_bincode_data");

	assertion(&Store
	{
		adapter: Adapters::Bincode,
		password: None,
		path: match temp_path.to_str()
		{
			Some(s) => s.into(),
			_ => Err(io::Error::new(
				io::ErrorKind::InvalidInput,
				"`env::temp_path` did not resolve to a valid path."
			)).unwrap(),
		},
		username: None,
	});
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
pub fn unique_id(store_dir: &Path) -> io::Result<Id>
{
	'gen: loop
	{
		let id = Id::new_v5(&UUID_NAMESPACE, Id::new_v4().as_bytes());

		for node_path in read_files(store_dir)?
		{
			if match node_path.file_stem()
			{
				Some(stem) => stem.to_string_lossy(),
				_ => continue,
			} == id.to_string()
			{ continue 'gen; }
		}

		return Ok(id);
	}
}

#[cfg(test)]
mod tests
{
	use
	{
		super::fs,
		std::{collections::HashSet, path::PathBuf, time::Instant},
	};

	#[test]
	fn test_unique_id()
	{
		super::test_temp_store(|store|
		{
			let test_path = PathBuf::new().join(&store.path).join("test_next_id");

			if test_path.is_dir()
			{
				fs::remove_dir_all(&test_path).unwrap();
			}

			// Create the `test_path`.
			super::create_store_dir(&test_path).unwrap();

			let mut ids = HashSet::new();
			let start = Instant::now();
			for _ in 0..100
			{
				let id = super::unique_id(&test_path).unwrap();
				ids.insert(id);

				// Creating the next file worked.
				assert!(fs::write(&test_path.join(id.to_string()), "TEST").is_ok());
			}
			println!("\n>>>>> util::uinque_id {}us <<<<<\n", Instant::now().duration_since(start).as_micros() / 100);

			// Assert that the number of unique IDs created is equal to the number of times looped.
			assert_eq!(ids.len(), 100);
		});
	}
}
