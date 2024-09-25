update
  nonprofit_clients
set
  email = $2,
  email_cc = $3,
  phone = $4,
  org_website = $5
where
  id = $1;

