# Commands

```sh
clinvoice --store default
                          config

                          create
                                 # will need to SELECT a `Location` when `--address`
                                 contact --label label --address # signifies that the contact is an `Address`
								  	                        --email "foo@bar.io" # signifies that the contact is an `Email`
								  									--phone "555-555-5555" # signifies that the contact is a `Phone`
								  									         "@foo" # signifies that the contact is an `Other`

                                 # will need to GENERATE `id`
                                 employee --name name --status status --title title

                                 # will need to GENERATE `id`
                                 # will need to SELECT `timesheet_id`
                                 expense --cateory category --cost="50.00 USD" --description="desc"

                                 # will need to GENERATE `id`
                                 # will need to SELECT `client`
                                 job --date-close="2022-01-01T00:00:00" --date-invoice-issued="2022-01-01T00:00:00" --date-invoice-paid="2022-01-01T00:00:00" --date-open="2022-01-01T00:00:00" --hourly-rate="50.00 USD" --increment 15min --notes="note" --objectives="objective"

                                 # will need to GENERATE `id`
                                 # will need to SELECT `outer` when `--inside`|`--outside` are `true`.
                                 location Phoenix Arizona USA # create the `Locations` and set "Earth" as an outermost location
											                             --inside # specify that "Earth" is inside another `Location`
                                                              --outside # specify that "Arizona" is outside another `Location`

                                 # will need to GENERATE `id`
                                 # will need to SELECT `location_id`
                                 organization --name name

                                 # will need to GENERATE `id`
                                 # will need to PROMPT to create `expenses`
                                 # will need to SELECT `employee` without `--default-employee`
                                 # will need to SELECT `job`
                                 timesheet --default-employee --work-notes="note" # implies `--time-begin=<now>` and no `--time-end`
                                                                                  --time-begin="2022-01-01T00:00:00" --time-end="2022-01-01T00:00:00"

                          delete --match foo.yml
                                                 contact
                                                 employee
                                                 expense
                                                 job
                                                 location
                                                 organization
                                                 timesheet

								  init

                          retrieve --match foo.yml
                                                   contact
                                                   employee --default
								  								            --set-default
                                                   expense
                                                   job
                                                       --export --format markdown --output-dir path/to/dir
                                                   location
                                                   organization --employer
                                                                --set-employer
                                                   timesheet

                          update --match foo.yml
                                                 contact
                                                 employee
                                                 expense
                                                 job --close
                                                     --reopen
                                                 location
                                                 organization
                                                 timesheet --restart
																           --stop
```
