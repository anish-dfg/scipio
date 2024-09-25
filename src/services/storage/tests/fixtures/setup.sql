-- Insert into project_cycles
insert into project_cycles(id, created_at, name, description, archived)
  values ('0e12b846-4de5-432e-8137-1bc2c92827b3', now(), 'Spring 2024', 'Spring 2024 project cycle', false),
('76ed64a0-d88f-4148-9b02-331ea888d5d1', now(), 'Fall 2024', 'Fall 2024 project cycle', false);

-- Insert into volunteers
insert into volunteers(id, created_at, project_cycle_id, first_name, last_name, email, phone, volunteer_gender, volunteer_ethnicity, volunteer_age_range, university, lgbt, country, us_state, fli, student_stage, majors, minors, hear_about)
  values ('1b1b5e16-d0d6-4ad1-8fdc-80df15b18b67', now(), '0e12b846-4de5-432e-8137-1bc2c92827b3', 'Rafael', 'Nadal', 'rafael.nadal@gmail.com', '123-456-7890', 'man', '{white_or_caucasian}', '35-39', '{"Stanford University"}', 'no', 'Spain', null, '{"prefer_not_to_say"}', 'recent_graduate', '{"Computer Science"}', '{"Mathematics"}', '{"linkedin"}'),
('9edc52d8-8cc7-4d44-80c1-7efcce246e90', now(), '0e12b846-4de5-432e-8137-1bc2c92827b3', 'Roger', 'Federer', 'roger.federer@gmail.com', '123-456-7891', 'man', '{white_or_caucasian}', '40-44', '{"Stanford University"}', 'no', 'Switzerland', null, '{"prefer_not_to_say"}', 'recent_graduate', '{"Computer Science"}', '{}', '{"linkedin"}'),
('5e7b3f35-2b84-46e7-8b7b-73e2716d42c9', now(), '76ed64a0-d88f-4148-9b02-331ea888d5d1', 'Novak', 'Djokovic', 'novak.djokovic@gmail.com', '123-456-7892', 'man', '{white_or_caucasian}', '40-44', '{"Harvard University"}', 'no', 'Serbia', null, '{"prefer_not_to_say"}', 'recent_graduate', '{"Computer Science"}', '{}', '{"linkedin"}'),
('0ef67e25-543c-4f0d-9a96-8cb71b3c0f60', now(), '0e12b846-4de5-432e-8137-1bc2c92827b3', 'Andy', 'Murray', 'andy.murray@gmail.com', '123-456-7893', 'man', '{white_or_caucasian}', '35-39', '{"Caltech", "University of California, Berkeley"}', 'ally', 'UK', null, '{"prefer_not_to_say"}', 'recent_graduate', '{"Computer Science"}', '{}', '{"linkedin"}'),
('fc32f485-4497-4ee9-a162-a5d12d6e737f', now(), '0e12b846-4de5-432e-8137-1bc2c92827b3', 'Stan', 'Wawrinka', 'stan.wawrinka@gmail.com', '123-456-7894', 'man', '{white_or_caucasian}', '35-39', '{"ETH Zurich"}', 'no', 'Switzerland', null, '{"prefer_not_to_say"}', 'recent_graduate', '{"Electrical Engineering"}', '{"Mathematics"}', '{"linkedin"}'),
('61596073-faf9-4cb5-b613-aa6b0ab86bb2', now(), '0e12b846-4de5-432e-8137-1bc2c92827b3', 'Dominic', 'Thiem', 'dominic.thiem@gmail.com', '123-456-7895', 'man', '{white_or_caucasian}', '25-29', '{"Massachusetts Institute of Technology"}', 'no', 'Austria', null, '{"prefer_not_to_say"}', 'recent_graduate', '{"Mathematics"}', '{"Data Science"}', '{"linkedin"}');

-- Insert into volunteer_team_roles
insert into volunteer_team_roles(project_cycle_id, volunteer_id, role_id)
  values ('0e12b846-4de5-432e-8137-1bc2c92827b3', '1b1b5e16-d0d6-4ad1-8fdc-80df15b18b67',(
      select
        id
      from
        team_roles
      where
        name = 'product_manager')),
('0e12b846-4de5-432e-8137-1bc2c92827b3',
    '9edc52d8-8cc7-4d44-80c1-7efcce246e90',
(
      select
        id
      from
        team_roles
      where
        name = 'engineer'));

-- Insert into mentors
insert into mentors(id, created_at, project_cycle_id, first_name, last_name, email, phone, company, job_title, country, us_state, years_experience, experience_level, prior_mentor, prior_mentee, prior_student, university, hear_about)
  values ('fa8377c8-1c0d-4f4e-9a2b-2c29f2737e0d', now(), '0e12b846-4de5-432e-8137-1bc2c92827b3', 'John', 'McEnroe', 'john.mcenroe@gmail.com', '123-456-7893', 'Team World', 'Coach', 'United States', 'New York', '21+', 'senior_or_executive', false, false, false, '{"Stanford University"}', '{"linkedin"}'),
('ab839d88-80c5-47f8-835a-1abbe269c7f8', now(), '76ed64a0-d88f-4148-9b02-331ea888d5d1', 'Bjorn', 'Borg', 'bjorn.borg@gmail.com', '123-456-7894', 'Team Europe', 'Coach', 'Sweden', null, '21+', 'senior_or_executive', false, false, false, '{"Harvard University"}', '{"linkedin"}');

-- Insert into nonprofit_clients
insert into nonprofit_clients(id, created_at, project_cycle_id, representative_first_name, representative_last_name, representative_job_title, email, email_cc, phone, org_name, project_name, org_website, country_hq, us_state_hq, address, size, impact_causes)
  values ('bb9b7fa5-7283-4b73-82e1-c7244e47421d', now(), '0e12b846-4de5-432e-8137-1bc2c92827b3', 'Petros', 'Sampras', 'CEO', 'pete.sampras@gmail.com', null, '123-456-7895', 'PeteOrg', 'PeteProject', 'www.peteorg.org', 'USA', null, '99999 Random Way', '21-50', '{"global_relations", "other"}'),
('dce8d3c3-8f2d-4f3e-80d3-c1633f5282e2', now(), '76ed64a0-d88f-4148-9b02-331ea888d5d1', 'Andre', 'Agassi', 'President', 'andre.agassi@gmail.com', null, '123-456-7896', 'AgassiOrg', 'AgassiProject', 'www.agassiorg.org', 'USA', null, '10000 Random Street', '101-500', '{"career_and_professional_development"}');

-- Insert into client_volunteers
insert into client_volunteers(created_at, volunteer_id, client_id, project_cycle_id, currently_active)
  values (now(), '1b1b5e16-d0d6-4ad1-8fdc-80df15b18b67', 'bb9b7fa5-7283-4b73-82e1-c7244e47421d', '0e12b846-4de5-432e-8137-1bc2c92827b3', true),
(now(), '9edc52d8-8cc7-4d44-80c1-7efcce246e90', 'bb9b7fa5-7283-4b73-82e1-c7244e47421d', '0e12b846-4de5-432e-8137-1bc2c92827b3', true),
(now(), '5e7b3f35-2b84-46e7-8b7b-73e2716d42c9', 'dce8d3c3-8f2d-4f3e-80d3-c1633f5282e2', '76ed64a0-d88f-4148-9b02-331ea888d5d1', true);

-- Insert into client_mentors
insert into client_mentors(created_at, mentor_id, client_id, project_cycle_id)
  values (now(), 'fa8377c8-1c0d-4f4e-9a2b-2c29f2737e0d', 'bb9b7fa5-7283-4b73-82e1-c7244e47421d', '0e12b846-4de5-432e-8137-1bc2c92827b3'),
(now(), 'ab839d88-80c5-47f8-835a-1abbe269c7f8', 'dce8d3c3-8f2d-4f3e-80d3-c1633f5282e2', '76ed64a0-d88f-4148-9b02-331ea888d5d1');

-- Insert into volunteer_mentors
insert into volunteer_mentors(mentor_id, volunteer_id, project_cycle_id)
  values ('fa8377c8-1c0d-4f4e-9a2b-2c29f2737e0d', '1b1b5e16-d0d6-4ad1-8fdc-80df15b18b67', '0e12b846-4de5-432e-8137-1bc2c92827b3'),
('fa8377c8-1c0d-4f4e-9a2b-2c29f2737e0d', '9edc52d8-8cc7-4d44-80c1-7efcce246e90', '0e12b846-4de5-432e-8137-1bc2c92827b3'),
('ab839d88-80c5-47f8-835a-1abbe269c7f8', '5e7b3f35-2b84-46e7-8b7b-73e2716d42c9', '76ed64a0-d88f-4148-9b02-331ea888d5d1');

insert into jobs(id, project_cycle_id, label, details, status)
  values ('bc080e0d-8b14-46e0-9268-4bbb370035ec', '0e12b846-4de5-432e-8137-1bc2c92827b3', 'Import base', '{"jobType": "airtable_import_base", "baseId": "appAasdawef"}', 'pending'),
('413eed73-3c6f-456a-b9f0-ae72d136c742', '76ed64a0-d88f-4148-9b02-331ea888d5d1', 'Export Users', '{"jobType": "export_users", "error": "Error serializing X at line Y", "export_destination": "google_workspace"}', 'error');

