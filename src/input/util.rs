pub mod contact;
pub mod employee;
pub mod expense;
pub mod job;
pub mod location;
mod menu;
pub mod organization;
pub mod person;

/// # Summary
///
/// The prompt for when editing a [query](clinvoice_query).
const QUERY_PROMPT: &str =
	"See the documentation of this query at https://github.com/Iron-E/clinvoice/wiki/Query-Syntax#";
