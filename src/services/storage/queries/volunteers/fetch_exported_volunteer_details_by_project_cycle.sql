select
  id,
  created_at,
  updated_at,
  volunteer_id,
  workspace_email,
  org_unit,
  job_id,
  project_cycle_id,
  status
from
  exported_volunteer_details
where
  project_cycle_id = $1
