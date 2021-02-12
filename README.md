# CLInvoice

## About

CLInvoice is a __WIP__ program to manage invoices from the command-line.

### Motivation

There is a lack of programs for CLI invoice maintenance, especially those which are able to export invoices in a presentable manner.

## Installation

### Cargo

1. Run the following command in a terminal:
	```sh
	cargo install clinvoice --features=<adapters>
	```
	* Any desired storage implementations (e.g. Bincode, PostreSQL) should be listed in place of `<adapters>`.


> NOTE: This application has not yet been uploaded to [crates.io](crates.io)!

#### Requirements

* `cargo`

### Source

1. Download this repository from GitHub:
	```sh
	git clone https://github.com/Iron-E/clinvoice
	```
2. Use `cargo` to build and install the cloned repo:
	```sh
	cargo install --path=./clinvoice
	```

#### Requirements

* `cargo`

## Supported Storage Adapters

Below is a list of supported storage adapters. If the adapter has a check next to it, the implementation is complete. If it has no check next to it, the implementation is either a WIP or planned.

* [x] Bincode File System
* [ ] PostgreSQL DB

## Usage

For more information, run `clinvoice help` from the command line.

# Roadmap

Below is a list of objectives which have been identified as necessary before this application's 1.0 release. Any item which is crossed out has been completed.

1. ~~Define the data model as `clinvoice_data`.~~
2. ~~Create adapter traits as `clinovice_adapter`.~~
3. ~~Implement `clinvoice_adapter` traits for bincode filesystem as `clinvoice_adapter_bincode`.~~
4. Write `clinvoice` application logic as `clinvoice_bin`.
5. Generate more boilerplate with `Adapt!` macro.
6. Write PostgreSQL statements for `clinvoice_data` entities.
7. Implement `clinovice_adapter` traits for PostgreSQL as `clinvoice_adapter_postgres`.
8. GUI as `guinvoice`?
