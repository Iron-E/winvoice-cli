//! `clinvoice_export` allows converting a [`Job`](clinvoice_schema::Job) into a [`String`] which
//! contains valid syntax for some specified file [`Format`].
//!
//! It is possible to write a new exporter as part of a custom CLInvoice frontend using the
//! modules in this crate.
//!
//! # Features
//!
//! * `markdown` enables [`Format::Markdown`].
//!
//! # See also
//!
//! * [`Format`], for the available file formats.
//! * [export_job][export] for more notes and warnings about the conversion.
//!
//! [export]: Format::export_job

mod format;

#[cfg(feature = "markdown")]
pub mod markdown;

#[cfg(not(any(feature = "markdown")))]
#[rustfmt::skip]
compile_error!("
`clinvoice_export` must be compiled with at least one of the following features:

* `markdown`
");

pub use format::Format;
