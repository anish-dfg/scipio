insert into jobs(project_cycle_id, label, description, details)
  values ($1, $2, $3, $4)
returning
  id;

