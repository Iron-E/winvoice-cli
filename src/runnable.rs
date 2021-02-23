use clinvoice_adapter::Store;

/// # Summary
///
/// Describes a structure which may be called upon to execute code like a main method.
pub trait Runnable<'pass, 'path, 'user>
{
	fn run(self, store: Store<'pass, 'path, 'user>);
}
