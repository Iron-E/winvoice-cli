use clinvoice_adapter::{
	schema::{
		ContactAdapter,
		EmployeeAdapter,
		ExpensesAdapter,
		JobAdapter,
		LocationAdapter,
		OrganizationAdapter,
		TimesheetAdapter,
	},
	Deletable,
};
use clinvoice_config::{Config, Error};
use clinvoice_schema::{chrono::Utc, ContactKind, Invoice, InvoiceDate};
use sqlx::{Database, Executor, Pool, Transaction};

use super::{Create, CreateCommand};
use crate::{
	args::{match_args::MatchArgs, update::Update, RunAction},
	input,
	utils,
	DynResult,
};

#[async_trait::async_trait(?Send)]
impl RunAction for Create
{
	async fn action<CAdapter, EAdapter, JAdapter, LAdapter, OAdapter, TAdapter, XAdapter, Db>(
		self,
		connection: Pool<Db>,
		config: Config,
	) -> DynResult<()>
	where
		CAdapter: Deletable<Db = Db> + ContactAdapter,
		EAdapter: Deletable<Db = Db> + EmployeeAdapter,
		JAdapter: Deletable<Db = Db> + JobAdapter,
		LAdapter: Deletable<Db = Db> + LocationAdapter,
		OAdapter: Deletable<Db = Db> + OrganizationAdapter,
		TAdapter: Deletable<Db = Db> + TimesheetAdapter,
		XAdapter: Deletable<Db = Db> + ExpensesAdapter,
		Db: Database,
		for<'connection> &'connection mut Db::Connection: Executor<'connection, Database = Db>,
		for<'connection> &'connection mut Transaction<'connection, Db>:
			Executor<'connection, Database = Db>,
	{
		match self.command
		{
			CreateCommand::Contact {
				label,
				address,
				email,
				phone,
				info,
			} =>
			{
				let kind = match (address, email, phone)
				{
					(true, ..) => input::select_one_retrieved::<LAdapter, _, _>(
						&connection,
						None,
						"Query the Location of this address",
					)
					.await
					.map(ContactKind::Address)?,
					(_, true, _) => ContactKind::Email(info),
					(.., true) => ContactKind::Phone(info),
					_ => ContactKind::Other(info),
				};

				let created = CAdapter::create(&connection, kind, label).await?;
				Self::report_created(&created);
			},

			CreateCommand::Employee {
				name,
				status,
				title,
			} =>
			{
				let created = EAdapter::create(&connection, name, status, title).await?;
				Self::report_created(&created);
			},

			CreateCommand::Expense {
				category,
				cost,
				description,
				timesheet,
			} =>
			{
				let match_timesheet = MatchArgs::from(timesheet).try_into()?;
				let selected = input::select_one_retrieved::<TAdapter, _, _>(
					&connection,
					match_timesheet,
					"Query the Timesheet this Expense is for",
				)
				.await?;

				#[rustfmt::skip]
				let created = XAdapter::create(&connection, vec![(category, cost, description)], selected.id)
					.await
					.map(|mut v| v.pop().expect("at least one `Expense` should have been created"))?;

				Self::report_created(&created);
			},

			CreateCommand::Job {
				client,
				date_close,
				date_invoice_issued,
				date_invoice_paid,
				date_open,
				employer,
				hourly_rate,
				increment,
				notes,
				objectives,
			} =>
			{
				let match_client = match employer
				{
					false => MatchArgs::from(client).try_into()?,
					_ => config
						.organizations
						.employer_id
						.ok_or_else(|| Error::NotConfigured("employer_id".into(), "organizations".into()))
						.map(|id| Some(id.into()))?,
				};

				let selected = input::select_one_retrieved::<OAdapter, _, _>(
					&connection,
					match_client,
					"Query the client Organization for this Job",
				)
				.await?;

				let created = JAdapter::create(
					&connection,
					selected,
					date_close.map(utils::naive_local_datetime_to_utc),
					date_open.map_or_else(Utc::now, utils::naive_local_datetime_to_utc),
					increment.unwrap_or(config.jobs.default_increment),
					Invoice {
						date: date_invoice_issued.map(|issued| InvoiceDate {
							issued: utils::naive_local_datetime_to_utc(issued),
							paid: date_invoice_paid.map(utils::naive_local_datetime_to_utc),
						}),
						hourly_rate,
					},
					notes,
					objectives,
				)
				.await?;

				Self::report_created(&created);
			},

			CreateCommand::Location {
				inside,
				outside,
				names,
			} =>
			{
				let mut names_reversed = names.into_iter().rev();

				let final_name = names_reversed
					.next()
					.expect("clap config should have ensured that `names` has length of at least one");

				let outside_of_final = match inside
				{
					true => input::select_one_retrieved::<LAdapter, _, _>(
						&connection,
						None,
						format!("Query the Location outside of {final_name}"),
					)
					.await
					.map(Some)?,
					_ => None,
				};

				// {{{
				let mut transaction = connection.begin().await?;

				// TODO: convert to `try_fold` after `stream`s merge to `std`? {{{2
				// TODO: use `inspect` after rust-lang/rust#91345
				let mut l = LAdapter::create(&mut *transaction, final_name, outside_of_final).await?;
				Self::report_created(&l);
				for n in names_reversed
				{
					l = LAdapter::create(&mut *transaction, n, Some(l)).await?;
					Self::report_created(&l);
				}
				// 2}}}

				let created = l;

				if outside
				{
					let mut inside_locations = input::select_retrieved::<LAdapter, _, _>(
						&connection,
						None,
						format!("Query Locations that are inside {created}"),
					)
					.await?;

					// PERF: only call `.clone` on the newly-`created` `Location` for elements in
					//       `inside_locations` other than the first
					if let Some(after_first) = inside_locations.get_mut(1..)
					{
						after_first.iter_mut().for_each(|mut l| {
							l.outer = Some(created.clone().into());
						})
					}

					if let Some(first) = inside_locations.first_mut()
					{
						first.outer = Some(created.into());
					}

					#[rustfmt::skip]
					LAdapter::update(
						&mut transaction,
						inside_locations.iter().inspect(|l| Update::report_updated(*l)),
					)
					.await?;
				}

				transaction.commit().await?;
				// }}}
			},

			CreateCommand::Organization { location, name } =>
			{
				let match_location = MatchArgs::from(location).try_into()?;
				let selected = input::select_one_retrieved::<LAdapter, _, _>(
					&connection,
					match_location,
					"Query the Location of this Organization",
				)
				.await?;

				let created = OAdapter::create(&connection, selected, name).await?;
				Self::report_created(&created);
			},

			CreateCommand::Timesheet {
				default_employee,
				employee,
				job,
				time_begin,
				time_end,
				work_notes,
			} =>
			{
				let match_employee = match default_employee
				{
					false => MatchArgs::from(employee).try_into()?,
					_ => config
						.employees
						.id
						.ok_or_else(|| Error::NotConfigured("id".into(), "employees".into()))
						.map(|id| Some(id.into()))?,
				};

				let employee = input::select_one_retrieved::<EAdapter, _, _>(
					&connection,
					match_employee,
					"Query the Employee who is responsible for the work",
				)
				.await?;

				let match_job = MatchArgs::from(job).try_into()?;
				let job = input::select_one_retrieved::<JAdapter, _, _>(
					&connection,
					match_job,
					"Query the Job being worked on",
				)
				.await?;

				let expenses = input::expense::menu()?;

				// {{{
				let mut transaction = connection.begin().await?;

				let created = TAdapter::create(
					&mut transaction,
					employee,
					expenses,
					job,
					time_begin.map_or_else(Utc::now, utils::naive_local_datetime_to_utc),
					time_end.map(utils::naive_local_datetime_to_utc),
					work_notes.unwrap_or_else(|| "None".into()),
				)
				.await?;

				transaction.commit().await?;
				// }}}

				Self::report_created(&created);
			},
		};

		Ok(())
	}
}
