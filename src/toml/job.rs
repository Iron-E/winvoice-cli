use chrono;

/// # Summary
///
/// A `Job` contains all of the information which pertains to the specific reasons that a client
/// has contacted the user's employer / the user.
///
/// It also defines the scope of the problem which is to be solved before an
/// [`Invoice`][invoice] is issued.
///
/// # Remarks
///
/// The `Job` can be thought of similarly to a support ticket. Whereas other structures may define
/// the method of payment, client information, and work periods— this structure defines what work
/// will _may_ performed.
///
/// [invoice]: super::invoice::Invoice
pub struct Job<'client, 'objectives, 'notes, Tz> where Tz : chrono::TimeZone
{
	/// # Summary
	///
	/// The number of the [`Job`].
	///
	/// # Remarks
	///
	/// Should be automatically generated, and __unique__, as it may be used for identifiaction
	/// purposes by clients.
	pub id: u64,

	/// # Summary
	///
	/// The name of the client who the work is being performed for.
	///
	/// # Todo
	///
	/// * Allow for creation and reference of clients independent of [`Job`]s.
	pub client_name: &'client str,

	/// # Summary
	///
	/// The date upon which the client requested the work.
	pub open_date: chrono::DateTime<Tz>,

	/// # Summary
	///
	/// The date upon which the client accepted the work as "complete".
	pub close_date: Option<chrono::DateTime<Tz>>,

	/// # Summary
	///
	/// What problems will be addressed before the [`Job`] is closed.
	///
	/// # Example
	///
	/// > __Note:__ the `str` may contain any valid markdown.
	///
	/// ```markdown
	/// * Fix website rendering issue.
	/// * Replace PDF with Google Form.
	/// * Contact customer support for X hardware device.
	/// ```
	pub objectives: &'objectives str,

	/// # Summary
	///
	/// Important things to know about the work that has been performed.
	///
	/// # Example
	///
	/// > __Note:__ the `str` may contain any valid markdown.
	///
	/// ```markdown
	/// * Images on the website now point to the correct location.
	/// * The PDF application has been replaced with a Google Form.
	/// * Customer support has been contacted and will reach out to you within X days.
	/// ```
	pub notes: &'notes str,
}
