use
{
	core::iter::FilterMap,
	std::
	{
		fs, io,
		path::{Path, PathBuf},
	},

	clinvoice_adapter::Store,
	clinvoice_data::{Id, UUID_NAMESPACE},

	serde::de::DeserializeOwned,
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
fn read_files<P>(path: P) -> io::Result<FilterMap<fs::ReadDir, impl FnMut(io::Result<fs::DirEntry>) -> Option<PathBuf>>> where
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

pub fn retrieve<E, T>(path: impl AsRef<Path>, query: impl Fn(&T) -> bool) -> Result<Vec<T>, E> where
	E : From<io::Error> + From<bincode::Error>,
	T : DeserializeOwned,
{
	let files = read_files(path)?;

	files.map(|file_path|
		{
			fs::File::open(file_path).map(|file| io::BufReader::new(file)).map_err(|e| e.into()).and_then(
				|reader|
				{
					let employee: Result<T, E> = bincode::deserialize_from(reader).map_err(|e| e.into());
					employee
				}
			)
		}
	).filter(|result| match result
	{
		Ok(employee) => query(&employee),
		_ => true, // errors should be included in the output
	}).collect()
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
pub fn temp_store(assertion: impl FnOnce(&Store))
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
				"`env::temp_path` did not resolve to a valid path"
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
	let files = read_files(store_dir)?.collect::<Vec<_>>();

	loop
	{
		let id = Id::new_v5(&UUID_NAMESPACE, Id::new_v4().as_bytes());
		let id_string = id.to_string();

		if files.iter()
			.flat_map(|file_path| file_path.file_stem())
			.all(|file_name| file_name.to_string_lossy() != id_string)
		{
			return Ok(id);
		}
	}
}

#[cfg(test)]
mod tests
{
	use
	{
		std::{collections::HashSet, time::Instant},

		super::{fs, PathBuf},
	};

	#[test]
	fn unique_id()
	{
		const LOOPS: usize = 1000;

		super::temp_store(|store|
		{
			let test_path = PathBuf::new().join(&store.path).join("test_next_id");

			if test_path.is_dir()
			{
				fs::remove_dir_all(&test_path).unwrap();
			}

			// Create the `test_path`.
			super::create_store_dir(&test_path).unwrap();

			let start = Instant::now();

			let ids = (0..LOOPS).fold(
				HashSet::new(),
				|mut s, _|
				{
					let id = super::unique_id(&test_path).unwrap();
					s.insert(id);

					// Creating the next file worked.
					assert!(fs::write(&test_path.join(id.to_string()), "TEST").is_ok());

					s
				}
			);

			println!("\n>>>>> util::unique_id {}us <<<<<\n", Instant::now().duration_since(start).as_micros() / (LOOPS as u128));

			// Assert that the number of unique IDs created is equal to the number of times looped.
			assert_eq!(ids.len(), LOOPS);
		});
	}
}
