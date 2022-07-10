//! Contains constants for various SQL keywords. Useful to prevent any possibility of misspelling
//! a keyword, or forgetting to insert spaces around keywords.
//!
//! # Examples
//!
//! Ideally, a misspelling like in `misspelling_compiles` would result in a compile-time error— any
//! SQL includes the output from `misspelling_compiles` would be invalid as well.
//!
//! ```rust
//! # use pretty_assertions::assert_eq;
//! fn misspelling_compiles(columns: &str, table: &str) -> String {
//!   format!("SELCT {columns} FROM{table}")
//! }
//! # assert_eq!(&misspelling_compiles("a, b, c, d", "foo"), "SELCT a, b, c, d FROMfoo");
//! ```
//!
//! ```rust,compile_fail
//! use clinvoice_adapter::fmt::sql::{SELECT, FROM};
//! fn misspelling_compiles_not(columns: &str, table: &str) -> String {
//!   format!("{SELCT}{columns}{FROM}{table}")
//! }
//! ```

pub const AND: &str = " AND ";
pub const AS: &str = " AS ";
pub const BETWEEN: &str = " BETWEEN ";
pub const CAST: &str = " CAST ";
pub const DELETE: &str = " DELETE ";
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
pub const SELECT: &str = " SELECT ";
pub const SET: &str = " SET ";
pub const TRUE: &str = " true ";
pub const UNION: &str = " UNION ";
pub const UPDATE: &str = " UPDATE ";
pub const WHERE: &str = " WHERE ";
pub const WITH_RECURSIVE: &str = " WITH RECURSIVE ";
