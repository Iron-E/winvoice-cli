use std::{
	fs,
	io,
	path::{Path, PathBuf},
};

use clinvoice_adapter::Store;
use clinvoice_data::{Id, UUID_NAMESPACE};
use serde::de::DeserializeOwned;
#[cfg(test)]
use {clinvoice_adapter::Adapters, std::env};

use crate::data::Result as DataResult;

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
	shellexpand::full(&store.path)
		.map(|p| p.as_ref().into())
		.unwrap_or_else(|_| store.path.as_str().into())
}

/// # Summary
///
/// Retrieves all [`T`]s from `path` where `query` is `true`.
///
/// # Errors
///
/// * If some [`fs::File`] in `path` is not a (valid) [`T`].
/// * When [`fs::read_dir`] does.
/// * When [`fs::File::open`] does.
pub fn retrieve<T>(
	path: impl AsRef<Path>,
	query: impl Fn(&T) -> DataResult<bool>,
) -> DataResult<Vec<T>>
where
	T: DeserializeOwned,
{
	let nodes = fs::read_dir(path)?;

	nodes
		.filter_map(|node| {
			node
				.ok()
				.map(|n| n.path())
				.filter(|node_path| node_path.is_file())
		})
		.map(|file_path| {
			fs::File::open(file_path)
				.map(io::BufReader::new)
				.map_err(|e| e.into())
				.and_then(|reader| {
					let employee: DataResult<T> =
						bincode::deserialize_from(reader).map_err(|e| e.into());
					employee
				})
		})
		.filter_map(|result| match result
		{
			Ok(t) => match query(&t)
			{
				Ok(b) if b => Some(Ok(t)),
				Err(e) => Some(Err(e)),
				_ => None,
			},
			Err(e) => Some(Err(e)),
		})
		.collect()
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

	assertion(&Store {
		adapter:  Adapters::Bincode,
		password: None,
		path:     match temp_path.to_str()
		{
			Some(s) => s.into(),
			_ => Err(io::Error::new(
				io::ErrorKind::InvalidInput,
				"`env::temp_path` did not resolve to a valid path",
			))
			.unwrap(),
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
	loop
	{
		let id = Id::new_v5(&UUID_NAMESPACE, Id::new_v4().as_bytes());

		if !store_dir.join(id.to_string()).is_file()
		{
			return Ok(id);
		}
	}
}

#[cfg(test)]
mod tests
{
	use std::{collections::HashSet, time::Instant};

	use super::{fs, PathBuf};

	#[test]
	fn unique_id()
	{
		const LOOPS: usize = 1000;

		super::temp_store(|store| {
			let test_path = PathBuf::new().join(&store.path).join("test_next_id");

			if test_path.is_dir()
			{
				fs::remove_dir_all(&test_path).unwrap();
			}

			// Create the `test_path`.
			super::create_store_dir(&test_path).unwrap();

			let start = Instant::now();

			let ids = (0..LOOPS).fold(HashSet::with_capacity(LOOPS), |mut s, _| {
				let id = super::unique_id(&test_path).unwrap();
				s.insert(id);

				// Creating the next file worked.
				assert!(fs::write(&test_path.join(id.to_string()), "TEST").is_ok());

				s
			});

			println!(
				"\n>>>>> util::unique_id {}us <<<<<\n",
				Instant::now().duration_since(start).as_micros() / (LOOPS as u128)
			);

			// Assert that the number of unique IDs created is equal to the number of times looped.
			assert_eq!(ids.len(), LOOPS);
		});
	}
}
