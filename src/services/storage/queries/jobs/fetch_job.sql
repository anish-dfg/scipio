select
  id,
  created_at,
  updated_at,
  project_cycle_id,
  status,
  label,
  description,
  details
from
  jobs
where
  id = $1;

