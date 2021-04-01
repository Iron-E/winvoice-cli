//! # Summary
//!
//! This module exports macros which correspond to common definitions of wrappers for
//! [clinvoice data](clinvoice_data). They can be used to expedite development of an adapter for
//! `clinvoice` and should be used where possible.

#![macro_use]

mod adapt;
mod adapt_employee;
mod adapt_job;
mod adapt_location;
mod adapt_organization;
mod adapt_person;
