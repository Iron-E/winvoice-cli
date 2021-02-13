mod create;
mod delete;
mod retrieve;
mod update;

/// # Summary
///
/// The entry-point for the `clinvoice` program.
pub enum Cli
{
	Create(Create),
	Delete(Delete),
	Retrieve(Retrieve),
	Update(Update),
}
