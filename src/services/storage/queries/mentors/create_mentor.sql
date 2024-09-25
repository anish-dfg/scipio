insert into mentors(project_cycle_id, first_name, last_name, email, phone, company, job_title, country, us_state, years_experience, experience_level, prior_mentor, prior_mentee, prior_student, university, hear_about)
  values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10::mentor_years_experience, $11::mentor_experience_level, $12, $13, $14, $15, $16)
returning
  id;

