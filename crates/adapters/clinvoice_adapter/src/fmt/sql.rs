//! Contains constants for various SQL keywords. Useful to prevent any possibility of misspelling
//! a keyword, or forgetting to insert spaces around keywords.
//!
//! # Examples
//!
//! ```rust
//! use clinvoice_adapter::fmt::sql;
//!
//! fn bad_example(columns: &str, table: &str) -> String {
//!   let mut example: String = "SELECT".into();
//!   example.push(columns);
//!   example.push("FROM");
//!   example.push(table);
//!   example
//! }
//!
//! fn good_example(columns: &str, table: &str) -> String {
//!   let mut example: String = sql::SELECT.into();
//!   example.push(columns);
//!   example.push(sql::FROM);
//!   example.push(table);
//!   example
//! }
//!
//! let columns = "a, b, c, d";
//! let table = "foo";
//! assert_eq!(&bad_example(columns, table), "SELECTa, b, c, dFROMfoo"); // oops, no spacing
//! assert_eq!(&good_example(columns, table), "SELECT a, b, c, d FROM foo"); // guaranteed correct
//! ```

pub const AND: &str = " AND ";
pub const AS: &str = " AS ";
pub const BETWEEN: &str = " BETWEEN ";
pub const CAST: &str = " CAST";
pub const DELETE_FROM: &str = "DELETE FROM ";
pub const EXISTS: &str = " EXISTS ";
pub const FALSE: &str = " false ";
pub const FROM: &str = " FROM ";
pub const GROUP_BY: &str = " GROUP BY ";
pub const IS: &str = " IS ";
pub const JOIN: &str = " JOIN ";
pub const LEFT: &str = " LEFT ";
pub const LIKE: &str = " LIKE ";
pub const NOT: &str = " NOT ";
pub const NULL: &str = " null ";
pub const OR: &str = " OR ";
pub const RETURNING: &str = " RETURNING ";
pub const SELECT: &str = "SELECT ";
pub const SET: &str = " SET ";
pub const TRUE: &str = " true ";
pub const UNION: &str = " UNION ";
pub const UPDATE: &str = "UPDATE ";
pub const WHERE: &str = " WHERE ";
pub const WITH_RECURSIVE: &str = "WITH RECURSIVE ";
