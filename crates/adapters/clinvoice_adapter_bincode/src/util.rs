use
{
	std::
	{
		io,
		path::{Path, PathBuf},
	},

	crate::data::{Error as DataError, Result as DataResult},

	clinvoice_adapter::Store,
	clinvoice_data::{Id, UUID_NAMESPACE},

	futures::{future, stream::TryStreamExt, TryFutureExt},
	serde::de::DeserializeOwned,
	tokio::{fs, io::AsyncReadExt},
	tokio_stream::wrappers::ReadDirStream,
};

#[cfg(test)]
use
{
	std::env,

	clinvoice_adapter::Adapters,

	futures::Future,
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
	shellexpand::full(&store.path).map(|p| p.as_ref().into()).unwrap_or_else(|_| store.path.into())
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
pub async fn retrieve<T>(path: impl AsRef<Path>, query: impl Fn(&T) -> DataResult<bool>) -> DataResult<Vec<T>> where
	T : DeserializeOwned,
{
	let node_results = fs::read_dir(path).map_err(DataError::from).await?;
	ReadDirStream::new(node_results).try_filter_map(|node|
	{
		let path = node.path();
		future::ok(if path.is_file() { Some(path) } else { None })
	}).err_into().and_then(|file_path| async move
	{
		let mut file = fs::File::open(file_path).await?;
		let mut contents = Vec::new();
		file.read_to_end(&mut contents).await?;
		bincode::deserialize::<T>(&contents).map_err(DataError::from)
	}).try_filter_map(|retrieval| match query(&retrieval)
	{
		Ok(b) if b => future::ok(Some(retrieval)),
		Err(e) => future::err(e),
		_ => future::ok(None),
	}).try_collect().await
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
pub async fn temp_store<F, Fut>(assertion: F) where
	F: FnOnce(&Store) -> Fut,
	Fut: Future<Output=()>,
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
	}).await;
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
	use
	{
		std::{collections::HashSet, time::Instant},

		super::{fs, PathBuf},
	};

	#[tokio::test]
	async fn unique_id()
	{
		const LOOPS: usize = 1000;

		super::temp_store(|store| async move
		{
			let test_path = PathBuf::new().join(&store.path).join("test_next_id");

			if test_path.is_dir()
			{
				fs::remove_dir_all(&test_path).await.unwrap();
			}

			// Create the `test_path`.
			super::create_store_dir(&test_path).await.unwrap();

			let start = Instant::now();

			use futures::stream::StreamExt;
			let ids = HashSet::with_capacity(LOOPS);
			futures::stream::iter(0..LOOPS).for_each_concurrent(None, |_| async move
			{
				let id = super::unique_id(&test_path).unwrap();
				ids.insert(id);

				// Creating the next file worked.
				assert!(fs::write(&test_path.join(id.to_string()), "TEST").await.is_ok());
			}).await;

			println!("\n>>>>> util::unique_id {}us <<<<<\n", Instant::now().duration_since(start).as_micros() / (LOOPS as u128));

			// Assert that the number of unique IDs created is equal to the number of times looped.
			assert_eq!(ids.len(), LOOPS);
		}).await
	}
}
