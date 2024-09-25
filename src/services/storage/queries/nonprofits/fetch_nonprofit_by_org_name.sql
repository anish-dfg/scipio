select
  client_id,
  created_at,
  updated_at,
  project_cycle_id,
  project_cycle_name,
  representative_first_name,
  representative_last_name,
  representative_job_title,
  email,
  email_cc,
  phone,
  org_name,
  project_name,
  org_website,
  country_hq,
  us_state_hq,
  address,
  size,
  impact_causes,
  volunteers,
  mentors
from
  nonprofit_client_details
where
  org_name = $1;

