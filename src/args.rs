mod command;
mod create;
mod delete;
mod flag_or_argument;
mod init;
mod match_args;
mod retrieve;
mod run_action;
mod store_args;
mod update;

use clap::Parser as Clap;
use clinvoice_config::Config;
use command::Command;
use dialoguer::Editor;
use run_action::RunAction;

use crate::DynResult;

/// CLInvoice is a tool to track and generate invoices from the command line. Pass --help for more.
///
/// It is capable of managing information about clients, employees, jobs, timesheets, and exporting
/// the information into the format of your choice.
#[derive(Clap, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Args
{
	/// The specific CLInvoice subcommand to run.
	#[clap(subcommand)]
	command: Command,
}

impl Args
{
	pub async fn run(self) -> DynResult<()>
	{
		let config = Config::read()?;

		match self.command
		{
			Command::Config =>
			{
				let serialized = toml::to_string_pretty(&config)?;
				if let Some(edited) = Editor::new().extension(".toml").edit(&serialized)?
				{
					let deserialized: Config = toml::from_str(&edited)?;
					deserialized.write()?;
				}
			},
			Command::Create(create) => create.run(config).await?,
			Command::Delete(delete) => delete.run(config).await?,
			Command::Init(init) => init.run(&config).await?,
			Command::Retrieve(retrieve) => retrieve.run(config).await?,
			Command::Update(update) => update.run(config).await?,
		};

		Ok(())
	}
}

#[cfg(test)]
mod tests
{
	use clap::Parser;

	use super::Args;

	#[test]
	fn try_parse()
	{
		/// An example date which can be deserialized into a
		/// [`clinvoice_schema::chrono::NaiveDateTime`].
		const DATE: &str = "2022-01-01T00:00:00";

		// sub-subcommands
		const CONTACT: &str = "contact";
		const EMPLOYEE: &str = "employee";
		const EXPENSE: &str = "expense";
		const JOB: &str = "job";
		const LOCATION: &str = "location";
		const ORGANIZATION: &str = "organization";
		const TIMESHEET: &str = "timesheet";

		/// Attempts to unwrap `parse_from` (since it gives better debug information than
		/// `assert!(result.is_ok())`/`is_err`).
		///
		/// Also does deduplication work; e.g. ensures optional flags are actually optional.
		macro_rules! unwrap
		{
			(create contact $($arg:expr) * $(, $err:ident)?) =>
			{
				unwrap!(create CONTACT $($arg) *, Err);
				unwrap!(create CONTACT $($arg) * "--label" "foo" $(, $err)?);
			};

			(create expense $($arg:expr) * $(, $err:ident)?) =>
			{
				unwrap!(create EXPENSE $($arg) * $(, $err)?);
				unwrap!(create EXPENSE $($arg) * "--timesheet" "path/to/timesheet.yaml" $(, $err)?);
			};

			(create location $($arg:expr) * $(, $err:ident)?) =>
			{
				unwrap!(create LOCATION $($arg) * $(, $err)?);

				unwrap!(create LOCATION $($arg) * "--inside" $(, $err)?);
				unwrap!(create LOCATION $($arg) * "--inside" "path" $(, $err)?);
				unwrap!(create LOCATION $($arg) * "--inside" "path" "--outside" $(, $err)?);
				unwrap!(create LOCATION $($arg) * "--inside" "path" "--outside" "path" $(, $err)?);
				unwrap!(create LOCATION $($arg) * "--inside"        "--outside" $(, $err)?);
				unwrap!(create LOCATION $($arg) * "--inside"        "--outside" "path" $(, $err)?);

				unwrap!(create LOCATION $($arg) *                   "--outside" $(, $err)?);
				unwrap!(create LOCATION $($arg) *                   "--outside" "path" $(, $err)?);
			};

			(create job -din $($arg:expr) * $(, $err:ident)?) =>
			{
				unwrap!(create job -di $($arg) * $(, $err)?);
				unwrap!(create job -di $($arg) * "--notes" "note" $(, $err)?);
			};

			(create job -di $($arg:expr) * $(, $err:ident)?) =>
			{
				unwrap!(create job -d $($arg) * $(, $err)?);
				unwrap!(create job -d $($arg) * "--increment" "15min" $(, $err)?);
			};

			(create job -d $($arg:expr) * $(, $err:ident)?) =>
			{
				unwrap!(create JOB $($arg) * $(, $err)?);
				unwrap!(create JOB $($arg) * "--date-open" DATE $(, $err)?);
				unwrap!(create JOB $($arg) * "--date-open" DATE "--date-close" DATE $(, $err)?);
				unwrap!(create JOB $($arg) * "--date-open" DATE "--date-close" DATE "--date-invoice-issued" DATE $(, $err)?);
				unwrap!(create JOB $($arg) * "--date-open" DATE "--date-close" DATE "--date-invoice-issued" DATE "--date-invoice-paid" DATE $(, $err)?);
				unwrap!(create JOB $($arg) * "--date-open" DATE "--date-close" DATE                              "--date-invoice-paid" DATE, Err);
				unwrap!(create JOB $($arg) * "--date-open" DATE                     "--date-invoice-issued" DATE, Err);
				unwrap!(create JOB $($arg) * "--date-open" DATE                     "--date-invoice-issued" DATE "--date-invoice-paid" DATE, Err);
				unwrap!(create JOB $($arg) * "--date-open" DATE                                                  "--date-invoice-paid" DATE, Err);
				unwrap!(create JOB $($arg) *                    "--date-close" DATE, Err);
				unwrap!(create JOB $($arg) *                    "--date-close" DATE "--date-invoice-issued" DATE, Err);
				unwrap!(create JOB $($arg) *                    "--date-close" DATE "--date-invoice-issued" DATE "--date-invoice-paid" DATE, Err);
				unwrap!(create JOB $($arg) *                    "--date-close" DATE                              "--date-invoice-paid" DATE, Err);
				unwrap!(create JOB $($arg) *                                        "--date-invoice-issued" DATE, Err);
				unwrap!(create JOB $($arg) *                                        "--date-invoice-issued" DATE "--date-invoice-paid" DATE, Err);
				unwrap!(create JOB $($arg) *                                                                     "--date-invoice-paid" DATE, Err);
			};

			(create job $($arg:expr) * $(, $err:ident)?) =>
			{
				unwrap!(create job -din $($arg) * $(, $err)?);
				unwrap!(create job -din $($arg) * "--client" "path" $(, $err)?);
				unwrap!(create job -din $($arg) * "--client" "path" "--employer", Err);
				unwrap!(create job -din $($arg) *                   "--employer" $(, $err)?);
			};

			(create organization $($arg:expr) * $(, $err:ident)?) =>
			{
				unwrap!(create ORGANIZATION $($arg) * $(, $err)?);
				unwrap!(create ORGANIZATION $($arg) * "--location" "path" $(, $err)?);
			};

			(create timesheet -j $($arg:expr) * $(, $err:ident)?) =>
			{
				unwrap!(create TIMESHEET $($arg) * $(, $err)?);
				unwrap!(create TIMESHEET $($arg) * "--job" "path" $(, $err)?);
				unwrap!(create TIMESHEET $($arg) * "--work-notes" "note" $(, $err)?);
			};

			(create timesheet $($arg:expr) * $(, $err:ident)?) =>
			{
				unwrap!(create timesheet -j $($arg) * $(, $err)?);
				unwrap!(create timesheet -j $($arg) * "--default-employee" $(, $err)?);
				unwrap!(create timesheet -j $($arg) * "--default-employee" "--employee" "path", Err);
				unwrap!(create timesheet -j $($arg) *                      "--employee" "path" $(, $err)?);
			};

			(update job $({-d $($date_arg:expr) +})? $($arg:literal) * $(, $err:ident)?) =>
			{
				unwrap!(update JOB $($($date_arg) +)?      $($arg) * $(, $err)?);
				unwrap!(update JOB $($($date_arg DATE) +)? $($arg) * $(, $err)?);
			};

			(update timesheet $({-d $($date_arg:expr) +})? $($arg:literal) * $(, $err:ident)?) =>
			{
				unwrap!(update TIMESHEET $($($date_arg) +)?      $($arg) * $(, $err)?);
				unwrap!(update TIMESHEET $($($date_arg DATE) +)? $($arg) * $(, $err)?);
			};

			(create   $($arg:expr) * $(, $err:ident)?) => { unwrap!("create"    -s $($arg) * $(, $err)?) };
			(delete   $($arg:expr) * $(, $err:ident)?) => { unwrap!("delete"   -ms $($arg) * $(, $err)?) };
			(init     $($arg:expr) * $(, $err:ident)?) => { unwrap!("init"      -s $($arg) * $(, $err)?) };
			(retrieve $($arg:expr) * $(, $err:ident)?) => { unwrap!("retrieve" -ms $($arg) * $(, $err)?) };
			(update   $($arg:expr) * $(, $err:ident)?) => { unwrap!("update"   -ms $($arg) * $(, $err)?) };

			($cmd:literal -ms $($arg:expr) * $(, $err:ident)?) =>
			{
				unwrap!($cmd -s                  $($arg) * $(, $err)?);
				unwrap!($cmd -s "--match" "path" $($arg) * $(, $err)?);
			};

			($cmd:literal -s $($arg:expr) * $(, $err:ident)?) =>
			{
				unwrap!($cmd                        $($arg) * $(, $err)?);
				unwrap!($cmd "--store" "some_store" $($arg) * $(, $err)?);
			};

			($($arg:expr) +)            => { unwrap!($($arg) +, unwrap) };
			($($arg:expr) +, Err)       => { unwrap!($($arg) +, unwrap_err) };
			($($arg:expr) +, $fn:ident) => { Args::try_parse_from(["clinvoice", $($arg),+]).$fn() };
		}

		// # clinvoice config
		unwrap!("config");

		// # clinvoice create
		unwrap!(create, Err);

		// # clinvoice create contact
		unwrap!(create contact, Err);
		unwrap!(create contact "info");

		unwrap!(create contact "--address");
		unwrap!(create contact "--address" stringify!(false));
		unwrap!(create contact "--address" stringify!(false) "info", Err);
		unwrap!(create contact "--address" "path");
		unwrap!(create contact "--address" "path" "info", Err);
		unwrap!(create contact "--address" stringify!(true));
		unwrap!(create contact "--address" stringify!(true) "info", Err);
		unwrap!(create contact "--address" "--email", Err);
		unwrap!(create contact "--address" "--email" "--phone", Err);
		unwrap!(create contact "--address"           "--phone", Err);

		unwrap!(create contact             "--email" "info");
		unwrap!(create contact             "--email" "--phone" "info", Err);

		unwrap!(create contact                       "--phone" "info");

		// # clinvoice create employee
		unwrap!(create EMPLOYEE, Err);

		unwrap!(create EMPLOYEE "--name" "first last", Err);
		unwrap!(create EMPLOYEE "--name" "first last" "--status" "yes", Err);
		unwrap!(create EMPLOYEE "--name" "first last" "--status" "yes" "--title" "nothx");
		unwrap!(create EMPLOYEE "--name" "first last"                  "--title" "nothx", Err);

		unwrap!(create EMPLOYEE                       "--status" "yes", Err);
		unwrap!(create EMPLOYEE                       "--status" "yes" "--title" "nothx", Err);

		unwrap!(create EMPLOYEE                                        "--title" "nothx", Err);

		// # clinvoice create expense
		unwrap!(create expense, Err);
		unwrap!(create expense "--category" "foo", Err);
		unwrap!(create expense "--category" "foo" "--cost" "20.00 USD", Err);
		unwrap!(create expense "--category" "foo" "--cost" "20.00 USD" "--description" "bar");
		unwrap!(create expense "--category" "foo"                      "--description" "bar", Err);
		unwrap!(create expense                    "--cost" "20.00 USD", Err);
		unwrap!(create expense                    "--cost" "20.00 USD" "--description" "bar", Err);
		unwrap!(create expense                                         "--description" "bar", Err);

		// clinvoice create job
		unwrap!(create job, Err);
		unwrap!(create job "--hourly-rate" "20.00 USD", Err);
		unwrap!(create job "--hourly-rate" "20.00 USD" "--objectives" "test");
		unwrap!(create job                             "--objectives" "test", Err);

		// # clinvoice create location
		unwrap!(create location, Err);
		unwrap!(create location "Arizona");
		unwrap!(create location "Desert View" "Phoenix" "Arizona" "USA");

		// # clinvoice create organization
		unwrap!(create organization, Err);
		unwrap!(create organization "--name" "first last");

		// # clinvoice create timesheet
		unwrap!(create timesheet);

		// # clinvoice delete
		unwrap!(delete, Err);
		unwrap!(delete CONTACT);
		unwrap!(delete EMPLOYEE);
		unwrap!(delete EXPENSE);
		unwrap!(delete JOB);
		unwrap!(delete LOCATION);
		unwrap!(delete ORGANIZATION);
		unwrap!(delete TIMESHEET);

		// # clinvoice init
		unwrap!(init);

		// # clinvoice retrieve
		unwrap!(retrieve, Err);

		// # clinvoice retrieve contact
		unwrap!(retrieve CONTACT);

		// # clinvoice retrieve employee
		unwrap!(retrieve EMPLOYEE);
		unwrap!(retrieve EMPLOYEE "--default");
		unwrap!(retrieve EMPLOYEE "--default" "--set-default", Err);
		unwrap!(retrieve EMPLOYEE             "--set-default");

		// # clinvoice retrieve expense
		unwrap!(retrieve EXPENSE);

		// # clinvoice retrieve job
		unwrap!(retrieve JOB);
		unwrap!(retrieve JOB "--export" "markdown");
		unwrap!(retrieve JOB "--export" "markdown" "--currency" "USD");
		unwrap!(retrieve JOB "--export" "markdown" "--currency" "USD" "--output-dir" "path/to/dir");
		unwrap!(retrieve JOB "--export" "markdown"                    "--output-dir" "path/to/dir");
		unwrap!(retrieve JOB                       "--currency" "USD", Err);
		unwrap!(retrieve JOB                       "--currency" "USD" "--output-dir" "path/to/dir", Err);
		unwrap!(retrieve JOB                                          "--output-dir" "path/to/dir", Err);

		// # clinvoice retrieve location
		unwrap!(retrieve LOCATION);

		// # clinvoice retrieve organization
		unwrap!(retrieve ORGANIZATION);
		unwrap!(retrieve ORGANIZATION "--employer");
		unwrap!(retrieve ORGANIZATION "--employer" "--set-employer", Err);
		unwrap!(retrieve ORGANIZATION              "--set-employer");

		// # clinvoice retrieve timesheet
		unwrap!(retrieve TIMESHEET);

		// # clinvoice update
		unwrap!(update, Err);

		// # clinvoice update contact
		unwrap!(update CONTACT);

		// # clinvoice update employee
		unwrap!(update EMPLOYEE);
		unwrap!(update EMPLOYEE "--default");

		// # clinvoice update expense
		unwrap!(update EXPENSE);

		// # clinvoice update job
		unwrap!(update job);

		unwrap!(update job {-d "--close"});
		unwrap!(update job {-d "--close"}                                            "--reopen", Err);
		unwrap!(update job {-d "--close" "--invoice-issued"});
		unwrap!(update job {-d "--close" "--invoice-issued"}                         "--reopen", Err);
		unwrap!(update job {-d "--close" "--invoice-issued" "--invoice-paid"});
		unwrap!(update job {-d "--close" "--invoice-issued" "--invoice-paid"}        "--reopen", Err);
		unwrap!(update job {-d "--close"                    "--invoice-paid"}, Err);
		unwrap!(update job {-d "--close"                    "--invoice-paid"}        "--reopen", Err);
		unwrap!(update job {-d           "--invoice-issued"});
		unwrap!(update job {-d           "--invoice-issued"}                         "--reopen", Err);
		unwrap!(update job {-d           "--invoice-issued" "--invoice-paid"});
		unwrap!(update job {-d           "--invoice-issued" "--invoice-paid"}        "--reopen", Err);
		unwrap!(update job {-d                              "--invoice-paid"});
		unwrap!(update job {-d                              "--invoice-paid"}        "--reopen", Err);
		unwrap!(update job                                                           "--reopen");

		// # clinvoice update location
		unwrap!(update LOCATION);

		// # clinvoice update organization
		unwrap!(update ORGANIZATION);
		unwrap!(update ORGANIZATION "--employer");

		// # clinvoice update timesheet
		unwrap!(update timesheet);
		unwrap!(update timesheet {-d "--restart"});
		unwrap!(update timesheet {-d "--restart" "--stop"}, Err);
		unwrap!(update timesheet {-d             "--stop"});
	}
}
