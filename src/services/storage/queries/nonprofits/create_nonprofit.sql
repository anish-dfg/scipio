insert into nonprofit_clients(project_cycle_id, representative_first_name, representative_last_name, representative_job_title, email, email_cc, phone, org_name, project_name, org_website, country_hq, us_state_hq, address, size, impact_causes)
  values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15::impact_cause[])
returning
  id;

