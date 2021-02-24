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

## Configuration

The first time that you run `clinvoice`, a configuration file will be created according to the table below:

| Platform | Value                                                                                     |
|:---------|:------------------------------------------------------------------------------------------|
| Linux    | `$XDG_CONFIG_HOME`__/clinvoice/config.toml__ or `$HOME`__/.config/clinvoice/config.toml__ |
| macOS    | `$HOME`__/Library/Application Support/clinvoice.toml__                                    |
| Windows  | `{FOLDERID_RoamingAppData}`__/clinvoice/config.toml__                                     |

This configuration file's supported options are:

```toml
[employees]
default_id = <your employee ID. this value should not be set manually, instead use: clinvoice retrieve employee --select-default>

[invoices]
default_currency = an ISO-4217 currency code, e.g. 'USD'

[stores]
default = an alias to a different adapter; e.g. 'foo'
foo = {
	adapter = a supported storage adapter; e.g. 'Bincode',
	password = Optional password. May or may not be accompanied by a username,
	path = Place where data can be found. Depends on the adapter— may be a path to a folder on a filesystem, or a schema on a database.,
	username = Optional username. May or may not be accompanied by a password,
}

[timesheets]
interval = amount of time; e.g. '5m', '3h', '10d 2s', etc
```

### Example

```toml
[employees]
default_id = '006c3a15-d8c9-4e9e-ba7f-c14846374101'

[invoices]
default_currency = 'USD'

[stores]
default = 'foo'
foo = {
	adapter = 'Bincode',
	path = '/home/Iron-E/Documents/CLInvoice',
}

[timesheets]
interval = '15m'
```

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
