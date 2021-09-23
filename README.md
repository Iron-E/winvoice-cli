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
2. `cd` to the directory which `git` just created.
3. Use `cargo` to build and install the cloned repo:
	```sh
	cargo install --features=<adapters> --path=. --root=<desired install folder>
	```

#### Requirements

* `cargo`

## Configuration

The first time that you run `clinvoice`, a configuration file will be created according to the table below:

| Platform | Value                                                                                     |
|:---------|:------------------------------------------------------------------------------------------|
| Linux    | `$XDG_CONFIG_HOME`__/clinvoice/config.toml__ or `$HOME`__/.config/clinvoice/config.toml__ |
| macOS    | `$HOME`__/Library/Application Support/clinvoice.toml__                                    |
| Windows  | `{FOLDERID_RoamingAppData}`__/clinvoice/config.toml__                                     |

Below is a summary of the configuration file's supported options. For a guide on configuring `store` adapters, see [here](https://github.com/Iron-E/clinvoice/wiki/Usage#adapters).

```toml
[employees]
default_id = # your employee ID. this value should not be set manually, instead use: `clinvoice retrieve employee --set-default`

[invoices]
default_currency = # what currency should be used if none is specified during `Job` creation.
                   # an ISO-4217 currency code, e.g. 'USD'

[stores]
default = # an alias to a different adapter; e.g. 'foo'.
foo = {
	adapter = # a supported storage adapter; e.g. 'Bincode'.
	url = # URL to connect to the database.
	      # See https://github.com/Iron-E/clinvoice/wiki/Usage#adapters
}

[timesheets]
default_interval = # Used if a specific `Interval`
                   # amount of time; e.g. '5min', '3h', '10d 2s', etc. See https://github.com/tailhook/humantime
```

### Example

See the [sample config](./SAMPLE_CONFIG.toml).

## Usage

For more information, run `clinvoice help` from the command line.

# Roadmap

Below is a list of objectives which have been identified as necessary before this application's 1.0 release. Any item which is crossed out has been completed.

1. ~~Define the data model as `clinvoice_data`.~~
2. ~~Create adapter traits as `clinovice_adapter`.~~
3. ~~Implement `clinvoice_adapter` traits for bincode filesystem as `clinvoice_adapter_bincode`.~~
4. Write `clinvoice` application logic as `clinvoice_bin`.
5. ~~Generate more boilerplate with `Adapt!` macro.~~
	* Refactored `clinvoice_adapter` to not require so much boilerplate.
6. Write PostgreSQL statements for `clinvoice_data` entities.
7. Implement `clinovice_adapter` traits for PostgreSQL as `clinvoice_adapter_postgres`.
8. GUI as `guinvoice`?
