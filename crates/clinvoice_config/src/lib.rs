//! `clinvoice_config` provides a definition for the [CLInvoice user config](Config), as well as
//! the ability to [read](Config::read) and [write](Config::write) it from/to disk.
//!
//! The user configuration is stored in the markup language __TOML__. If you are unfamiliar, you
//! can learn about it [here](https://toml.io/).

mod adapters;
mod config;
mod employees;
mod error;
mod invoices;
mod jobs;
mod store;
mod store_value;

pub use adapters::Adapters;
pub use config::Config;
pub use employees::Employees;
pub use error::{Error, Result};
pub use invoices::Invoices;
pub use jobs::Jobs;
pub use store::Store;
pub use store_value::StoreValue;
