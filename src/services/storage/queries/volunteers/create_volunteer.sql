insert into volunteers(project_cycle_id, first_name, last_name, email, phone, volunteer_gender, volunteer_ethnicity, volunteer_age_range, university, lgbt, country, us_state, fli, student_stage, majors, minors, hear_about)
  values ($1, $2, $3, $4, $5, $6::gender, $7::ethnicity[], $8::age_range, $9, $10::lgbt_status, $11, $12, $13::fli_status[], $14::student_stage, $15, $16, $17)
returning
  id;

