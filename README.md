# CLInvoice

## About

CLInvoice is a __WIP__ program to manage invoices from the command-line.

### Motivation

There is a lack of programs for CLI invoice maintenance, especially those which are able to export invoices in a presentable manner.

## Installation

1. __TODO__

### Requirements

* Rust (__TODO__)

## Usage

* [ ] Help
	```sh
	clinvoice help <command>
	```
* [ ] Export
	```sh
	clinvoice export <input_file> [-o|--output <output_file>]
	```
* [ ] List
	```sh
	clinvoice list [-c|--client <client_name>] [-i|--issued|--no-issued] [-o|--outstanding|--no-outstanding] [-s|--sort <sort_by>]
	```
* [ ] New
	```sh
	clinvoice new <client_name> [-r|--rate <currency_symbol><job_rate>]
	```
* [ ] Manipulate timesheets:
	```sh
	clinvoice time [start|stop] <job_number>
	```

## Configuration

* [ ] List configuration:
	```sh
	clinvoice config
	```
* [ ] Reset to defaults:
	```sh
	clinvoice config -r|--reset
	```
* [ ] Invoice directory:
	```sh
	clinvoice config -d|--directory <invoice_directory>
	```
* [ ] Default company street address:
	```sh
	clinvoice config -a|--address <company_address>
	```
* [ ] Default company email:
	```sh
	clinvoice config -m|--email <company_email>
	```
* [ ] Default company name:
	```sh
	clinvoice config -n|--name <company_name>
	```
* [ ] Default technician name:
	```sh
	clinvoice config -t|--technician <technician_name>
	```
* [ ] Track time in specific intervals (less than `1` to disable):
	```sh
	clinvoice config -i|--interval <integer>
	```
