//! CLInvoice is a __WIP__ program to manage invoices from the command-line.
//!
//! # Motivation
//!
//! There is a lack of programs for CLI invoice maintenance, especially those which are able to export
//! invoices in a presentable manner.
//!
//! # Installation
//!
//! Requirements:
//!
//! * `cargo`
//!
//! ## [crates.io](https://crates.io)
//!
//! Run the following command in a terminal:
//!
//! ```sh
//! cargo install clinvoice --features=<adapters>
//! ```
//!
//! * Any desired storage implementations (e.g. Bincode, PostreSQL) should be listed in place of
//!   `<adapters>`.
//!
//! > __NOTE__: This application has not yet been uploaded to [crates.io](crates.io)!
//!
//! ## Source
//!
//! 1. Download this repository from GitHub:
//!    ```sh
//!    git clone https://github.com/Iron-E/clinvoice
//!    ```
//! 2. `cd` to the directory which `git` just created.
//! 3. Use `cargo` to build and install the cloned repo:
//!    ```sh
//!    cargo install --features=<adapters> --path=. --root=<desired install folder>
//!    ```
//!
//! # Configuration
//!
//! Run `clinvoice config` to edit the configuration file for this program. See the documentation
//! of [`Config`](clinvoice_config::Config) for more information about the configuration file and
//! its options.
//!
//! # Usage
//!
//! * For basic information, run `clinvoice help` from the command line.
//! * For an in-depth guide, see the [wiki](https://github.com/Iron-E/clinvoice/wiki/Usage).

#![allow(clippy::drop_non_drop)]
#![forbid(unsafe_code)]
#![warn(
	missing_docs,
	clippy::alloc_instead_of_core,
	clippy::allow_attributes_without_reason,
	clippy::as_underscore,
	clippy::branches_sharing_code,
	clippy::cast_lossless,
	clippy::checked_conversions,
	clippy::cloned_instead_of_copied,
	clippy::dbg_macro,
	clippy::debug_assert_with_mut_call,
	clippy::doc_link_with_quotes,
	clippy::doc_markdown,
	clippy::empty_line_after_outer_attr,
	clippy::empty_structs_with_brackets,
	clippy::enum_glob_use,
	clippy::equatable_if_let,
	clippy::exit,
	clippy::explicit_into_iter_loop,
	clippy::explicit_iter_loop,
	clippy::fallible_impl_from,
	clippy::filetype_is_file,
	clippy::filter_map_next,
	clippy::flat_map_option,
	clippy::fn_to_numeric_cast_any,
	clippy::format_push_string,
	clippy::from_iter_instead_of_collect,
	clippy::get_unwrap,
	clippy::implicit_clone,
	clippy::inefficient_to_string,
	clippy::items_after_statements,
	clippy::manual_assert,
	clippy::manual_ok_or,
	clippy::map_unwrap_or,
	clippy::match_same_arms,
	clippy::missing_const_for_fn,
	clippy::missing_panics_doc,
	clippy::multiple_inherent_impl,
	clippy::mut_mut,
	clippy::needless_continue,
	clippy::option_if_let_else,
	clippy::option_option,
	clippy::range_minus_one,
	clippy::range_plus_one,
	clippy::redundant_closure_for_method_calls,
	clippy::redundant_else,
	clippy::ref_binding_to_reference,
	clippy::ref_option_ref,
	clippy::same_functions_in_if_condition,
	clippy::single_char_lifetime_names,
	clippy::std_instead_of_core,
	clippy::str_to_string,
	clippy::string_add,
	clippy::string_add_assign,
	clippy::string_to_string,
	clippy::try_err,
	clippy::unnecessary_join,
	clippy::unnecessary_wraps,
	clippy::use_self,
	clippy::used_underscore_binding,
	clippy::wildcard_imports
)]

mod args;
mod dyn_result;
mod fmt;
mod input;
mod utils;

use args::Args;
use clap::Parser;
use dyn_result::DynResult;

/// # Summary
///
/// The main method.
#[tokio::main]
async fn main() -> DynResult<()>
{
	Args::parse().run().await
}
