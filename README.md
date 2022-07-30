# CLInvoice

<!-- cargo-rdme start -->

CLInvoice is a __WIP__ program to manage invoices from the command-line.

## Motivation

There is a lack of programs for CLI invoice maintenance, especially those which are able to export invoices in a presentable manner.

## Installation

Requirements:

* `cargo`

### [crates.io][crates]

Run the following command in a terminal:

```sh
cargo install clinvoice --features=<adapters>
```

* Any desired storage implementations (e.g. Bincode, PostgreSQL) should be listed in place of `<adapters>`.

> __NOTE__: This application has not yet been uploaded to [crates.io][crates]!

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

## Configuration

Run `clinvoice config` to edit the configuration file for this program. See the documentation of [`Config`](clinvoice_config::Config) for more information about the configuration file and its options.

## Usage

* For basic information, run `clinvoice help` from the command line.
* For an in-depth guide, see the [wiki](https://github.com/Iron-E/clinvoice/wiki/Usage).

[crates]: https://crates.io

<!-- cargo-rdme end -->
