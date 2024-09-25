update
  mentors
set
  first_name = $2,
  last_name = $3,
  email = $4,
  phone = $5
where
  id = $1;

