# Commands

```sh
clinvoice --store="default"
                            create
                                   # will need to SELECT a `Location` when `--address`
                                   contact --label="label" --address --email="foo@bar.io" --phone="555-555-5555" --other="@foo"

                                   # will need to GENERATE `id`
                                   employee --name="name" --status="status" --title="title"

                                   # will need to GENERATE `id`
                                   # will need to SELECT `timesheet_id`
                                   expense --cateory="category" --cost="50.00 USD" --description="desc"

                                   # will need to GENERATE `id`
                                   # will need to SELECT `client`
                                   job --date-close="2022-01-01T00:00:00" --date-invoice-issued="2022-01-01T00:00:00" --date-invoice-paid="2022-01-01T00:00:00" --date-open="2022-01-01T00:00:00" --notes="note" --objectives="objective" --increment="15min" --hourly-rate="50.00 USD"

                                   # will need to GENERATE `id`
                                   # will need to SELECT `outer` when `--inside`|`--outside` are `true`.
                                   location --name="Arizona" --inside # specify that "Arizona" is inside another `Location`
                                                             --inside="USA" --inside="Earth" # create the all `Location`s outside `Arizona`
                                                             --outside # specify that "Arizona" is outside another `Location`

                                   # will need to GENERATE `id`
                                   # will need to SELECT `location_id`
                                   organization --name="name"

                                   # will need to GENERATE `id`
                                   # will need to PROMPT to create `expenses`
                                   # will need to SELECT `employee` without `--default-employee`
                                   # will need to SELECT `job`
                                   timesheet --default-employee --work-notes="note" # implies `--time-begin=<now>` and no `--time-end`
                                                                                    --time-begin="2022-01-01T00:00:00" --time-end="2022-01-01T00:00:00"

                            delete --match="foo.yml"
                                                     contact
                                                     employee
                                                     expense
                                                     job
                                                     location
                                                     organization
                                                     timesheet

                            retrieve --match="foo.yml"
                                                       contact
                                                       employee --default
																		          --set-default
                                                       expense
                                                       job --export
                                                       location
                                                       organization --employer
																		              --set-employer
                                                       timesheet

                            update --match="foo.yml"
                                                     contact
                                                     employee
                                                     expense
                                                     job --close
																			--reopen
                                                     location
                                                     organization
                                                     timesheet --stop
```
