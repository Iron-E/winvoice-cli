//! # Summary
//!
//! This module implements adapters (and associated adapter types such as
//! [`Deletable`](clinvoice_adapter::data::Deletable)) for a Bincode filesystem.

mod employee;
mod error;
mod job;
mod location;
mod organization;
mod person;

pub use employee::BincodeEmployee;
pub use error::{
	Error,
	Result,
};
pub use job::BincodeJob;
pub use location::BincodeLocation;
pub use organization::BincodeOrganization;
pub use person::BincodePerson;
