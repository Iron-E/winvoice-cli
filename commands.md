# Commands

```sh
winvoice
                          config

                          create --store default
                                 # will need to SELECT a `Location` when `--address`
                                 contact --label label --address # signifies that the contact is an `Address`
                                                       --address [path/to/location.yaml] # signifies that the contact is an `Address`
                                                       --email "foo@bar.io" # signifies that the contact is an `Email`
                                                       --phone "555-555-5555" # signifies that the contact is a `Phone`
                                                               "@foo" # signifies that the contact is an `Other`

                                 # will need to GENERATE `id`
                                 employee --name name --status status --title title

                                 # will need to GENERATE `id`
                                 # will need to SELECT `timesheet_id`
                                 expense --cateory category --cost "50.00 USD" --description "desc" --timesheet path/to/timesheet.yaml

                                 # will need to GENERATE `id`
                                 # will need to SELECT `client`
                                 job --date-close "2022-01-01T00:00:00" --date-invoice-issued "2022-01-01T00:00:00" --date-invoice-paid "2022-01-01T00:00:00" --date-open "2022-01-01T00:00:00" --employer --hourly-rate "50.00 USD" --increment 15min --notes "note" --objectives "objective"

                                 # will need to GENERATE `id`
                                 # will need to SELECT `outer` when `--inside`|`--outside` are `true`.
                                 location Phoenix Arizona USA # create the `Locations` and set "Earth" as an outermost location
                                                              --inside # specify that "Earth" is inside another `Location`
                                                              --outside # specify that "Arizona" is outside another `Location`

                                 # will need to GENERATE `id`
                                 # will need to SELECT `location_id`
                                 organization --name name --location path/to/location.yaml

                                 # will need to GENERATE `id`
                                 # will need to PROMPT to create `expenses`
                                 # will need to SELECT `employee` without `--default-employee`
                                 # will need to SELECT `job`
                                 timesheet --work-notes "note" # implies `--time-begin <now>` and no `--time-end`
                                                               --default-employee
                                                               --employee path/to/file.yaml
                                                               --job path/to/file.yaml
                                                               --time-begin "2022-01-01T00:00:00" --time-end "2022-01-01T00:00:00"

                          delete --match foo.yml --store default
                                 contact
                                 employee
                                 expense
                                 job
                                 location
                                 organization
                                 timesheet

                          init --store default

                          retrieve --match foo.yml --store default
                                   contact
                                   employee --default
                                            --set-default
                                   expense
                                   job --export markdown --currency USD --output-dir path/to/dir
                                   location
                                   organization --employer
                                                --set-employer
                                   timesheet

                          update --match foo.yml --store default
                                 contact
                                 employee --default
                                 expense
                                 job --close
                                     --invoice-issued
                                     --invoice-paid
                                     --reopen
                                 location
                                 organization --employer
                                 timesheet --restart
                                           --stop
```
