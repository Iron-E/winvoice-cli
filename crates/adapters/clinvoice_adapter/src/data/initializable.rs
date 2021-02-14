use
{
	crate::Store,
	std::error::Error,
};

pub trait Initializable<'pass, 'path, 'user>
{
	/// # Summary
	///
	/// Initialize the database for a given [`Store`].
	fn init(store: &Store<'pass, 'path, 'user>) -> Result<(), Box<dyn Error>>;
}
