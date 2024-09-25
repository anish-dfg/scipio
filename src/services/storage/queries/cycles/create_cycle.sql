insert into project_cycles(name, description)
  values ($1, $2)
returning
  id;

