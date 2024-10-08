select
  volunteer_id,
  created_at,
  updated_at,
  project_cycle_id,
  project_cycle_name,
  first_name,
  last_name,
  email,
  phone,
  volunteer_gender,
  volunteer_ethnicity,
  volunteer_age_range,
  university,
  lgbt,
  country,
  us_state,
  fli,
  student_stage,
  majors,
  minors,
  hear_about,
  clients,
  mentors,
  workspace_email,
  roles
from
  volunteer_details
where
  project_cycle_id = $1;

