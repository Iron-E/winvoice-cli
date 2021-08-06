use std::{
	io,
	path::{Path, PathBuf},
};

use clinvoice_adapter::Store;
use clinvoice_data::{Id, UUID_NAMESPACE};
use futures::{future, stream::TryStreamExt, TryFutureExt};
use serde::de::DeserializeOwned;
use tokio::{fs, io::AsyncReadExt};
use tokio_stream::wrappers::ReadDirStream;
#[cfg(test)]
use {clinvoice_adapter::Adapters, std::env};

use crate::data::{Error as DataError, Result as DataResult};

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
pub async fn create_store_dir(store_dir: &Path) -> io::Result<()>
{
	if !store_dir.is_dir()
	{
		fs::create_dir_all(store_dir).await?;
	}

	Ok(())
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
pub async fn retrieve<T>(
	path: impl AsRef<Path>,
	query: impl Fn(&T) -> DataResult<bool>,
) -> DataResult<Vec<T>>
where
	T: DeserializeOwned,
{
	let node_results = fs::read_dir(path).map_err(DataError::from).await?;
	ReadDirStream::new(node_results)
		.try_filter_map(|node| {
			let path = node.path();
			future::ok(if path.is_file() { Some(path) } else { None })
		})
		.err_into()
		.map_ok(|file_path| async {
			let mut file = fs::File::open(file_path).await?;
			let mut contents = Vec::new();
			file.read_to_end(&mut contents).await?;
			bincode::deserialize::<T>(&contents).map_err(DataError::from)
		})
		.try_buffer_unordered(10)
		.try_filter_map(|retrieval| match query(&retrieval)
		{
			Ok(b) if b => future::ok(Some(retrieval)),
			Err(e) => future::err(e),
			_ => future::ok(None),
		})
		.try_collect()
		.await
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
pub fn temp_store() -> Store
{
	let temp_path = env::temp_dir().join("clinvoice_adapter_bincode_data");

	Store {
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
	}
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
	use std::{collections::HashSet, fs, time::Instant};

	use super::PathBuf;

	/// NOTE: this test is `async` because of the single `create_store_dir` call.
	/// TODO: see if `Stream`ing to an `Arc<Mutex<HashSet>>` would make this faster
	#[tokio::test(flavor = "multi_thread", worker_threads = 10)]
	async fn unique_id()
	{
		const LOOPS: usize = 1000;
		let store = super::temp_store();

		let test_path = PathBuf::new().join(&store.path).join("test_next_id");

		if test_path.is_dir()
		{
			fs::remove_dir_all(&test_path).unwrap();
		}

		// Create the `test_path`.
		super::create_store_dir(&test_path).await.unwrap();

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
	}
}
