use
{
	std::{env, io},

	clinvoice_adapter::{Adapters, Store},
};

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
