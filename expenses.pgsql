SELECT
	T.id as timesheet_id,
	X.category,
	X.cost,
	X.description,
	X.id
FROM timesheets T
LEFT JOIN expenses X ON (X.timesheet_id = T.id)
WHERE NOT EXISTS
(
	SELECT FROM expenses X_2
	WHERE X_2.timesheet_id = X.timesheet_id
)
ORDER BY
	timesheet_id
;
