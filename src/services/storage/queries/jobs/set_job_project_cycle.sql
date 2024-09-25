update
  jobs
set
  project_cycle_id = $2
where
  id = $1;

