use const_format::concatcp;

/* GENERAL */

const FIELDS: &str = include_str!("yaml/fields.ungram");
const MATCH: &str = concatcp!(include_str!("yaml/match.ungram"), "\n", FIELDS);
const TYPES: &str = include_str!("yaml/types.ungram");

/* TYPE QUERIES */

const DATE: &str = include_str!("yaml/string.ungram"); // requires `MATCH` and `TYPES`
const STRING: &str = include_str!("yaml/string.ungram"); // requires `MATCH` and `TYPES`
const UUID: &str = include_str!("yaml/uuid.ungram"); // requires `MATCH` and `TYPES`

/* CLINVOICE QUERIES */

const PERSON_UNGRAMMAR: &str = include_str!("yaml/person.ungram");

/// # Summary
///
/// Get the reference for a YAML [`crate::data::query::Employee`].
pub const EMPLOYEE: &str = concatcp!(
	include_str!("yaml/employee.ungram"), "\n",
	include_str!("yaml/contact.ungram"), "\n",
	PERSON_UNGRAMMAR, "\n",
	ORGANIZATION,
);

/// # Summary
///
/// Get the reference for a YAML [`crate::data::query::Job`].
pub const JOB: &str = concatcp!(
	include_str!("yaml/job.ungram"), "\n",
	include_str!("yaml/invoice.ungram"), "\n",
	include_str!("yaml/invoice_date.ungram"), "\n",
	TIMESHEET,
);

/// # Summary
///
/// Get the reference for a YAML [`crate::data::query::Location`].
pub const LOCATION: &str = concatcp!(
	include_str!("yaml/location.ungram"), "\n",
	STRING, "\n",
	UUID, "\n",
	MATCH, "\n",
	TYPES,
);

/// # Summary
///
/// Get the reference for a YAML [`crate::data::query::Organization`].
pub const ORGANIZATION: &str = concatcp!(
	include_str!("yaml/organization.ungram"), "\n",
	LOCATION,
);

/// # Summary
///
/// Get the reference for a YAML [`crate::data::query::Person`].
pub const PERSON: &str = concatcp!(
	PERSON_UNGRAMMAR, "\n",
	STRING, "\n",
	UUID, "\n",
	MATCH, "\n",
	TYPES,
);

/// # Summary
///
/// Get the reference for a YAML [`crate::data::query::Timesheet`].
pub const TIMESHEET: &str = concatcp!(
	include_str!("yaml/timesheet.ungram"), "\n",
	DATE, "\n",
	EMPLOYEE,
);
