-- Add down migration script here
--
-- Drop functions and triggers in reverse order of creation
--
drop view if exists nonprofit_client_details;

drop view if exists volunteer_details;

drop trigger if exists set_updated_at on client_volunteers;

drop trigger if exists set_updated_at on airtable_user_export_jobs;

drop trigger if exists set_updated_at on airtable_data_import_jobs;

drop trigger if exists set_updated_at on volunteer_mentors;

drop trigger if exists set_updated_at on client_mentors;

drop trigger if exists set_updated_at on clients;

drop trigger if exists set_updated_at on mentors;

drop trigger if exists set_updated_at on team_roles;

drop trigger if exists set_updated_at on volunteers;

drop trigger if exists set_updated_at on project_cycles;

drop function if exists trigger_updated_at(regclass);

drop function if exists set_updated_at();

--
-- Drop tables and types in reverse order of creation
--
drop table if exists workspace_volunteer_exports;

drop table if exists airtable_user_export_jobs;

drop table if exists airtable_data_import_jobs;

drop table if exists volunteer_mentors;

drop table if exists client_mentors;

drop table if exists client_volunteers;

drop table if exists clients;

drop table if exists mentors;

drop table if exists volunteer_team_roles;

drop table if exists team_roles;

drop table if exists volunteers;

drop table if exists project_cycles;

--
-- Drop enum types in reverse order of creation
--
drop type if exists gender;

drop type if exists job_status;

drop type if exists nonprofit_size;

drop type if exists ethnicity;

drop type if exists age_range;

drop extension if exists "uuid-ossp";

