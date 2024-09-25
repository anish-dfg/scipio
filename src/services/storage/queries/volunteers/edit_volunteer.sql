update
  volunteers
set
  email = $2,
  phone = $3
where
  id = $1;

