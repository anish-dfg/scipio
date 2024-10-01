-- Add up migration script here
create extension if not exists "uuid-ossp";

-- function to update timestamps if a table has an updated_at column
create or replace function set_updated_at()
  returns trigger
  as $$
begin
  new.updated_at = now();
  return NEW;
end;
$$
language plpgsql;

-- automate trigger creation
create or replace function trigger_updated_at(tablename regclass)
  returns void
  as $$
begin
  execute format('create trigger set_updated_at
        before update
        on %s
        for each row
        when (OLD is distinct from NEW)
    execute function set_updated_at();', tablename);
end;
$$
language plpgsql;

-- Allowed age ranges for volunteers (from Airtable)
create type age_range as enum(
  '18-24',
  '25-29',
  '30-34',
  '35-39',
  '40-44',
  '45-59',
  '60-64',
  '65+',
  'prefer_not_to_say'
);

-- Allowed ethnicity values for volunteers (from Airtable)
create type ethnicity as enum(
  'asian',
  'white_or_caucasian',
  'black_or_african_american',
  'american_indian_or_alaska_native',
  'native_hawaiian_or_pacific_islander',
  'latino_or_hispanic',
  'other',
  'prefer_not_to_say'
);

-- Allowed values for a nonprofit's size (from Airtable)
create type client_size as enum(
  '0',
  '1-5',
  '6-20',
  '21-50',
  '51-100',
  '101-500',
  '500+'
);

create type gender as enum(
  'woman',
  'man',
  'non_binary',
  'other',
  'prefer_not_to_say'
);

-- Allowed states for asynchronous jobs
create type job_status as enum(
  'pending',
  'complete',
  'cancelled',
  'error'
);

create type lgbt_status as enum(
  'yes',
  'no',
  'ally',
  'prefer_not_to_say'
);

create type fli_status as enum(
  'first_generation',
  'low_income',
  'neither',
  'prefer_not_to_say'
);

create type student_stage as enum(
  'freshman',
  'sophomore',
  'junior',
  'senior',
  'masters_student',
  'phd_student',
  'recent_graduate'
);

create type mentor_years_experience as enum(
  '2-5',
  '6-10',
  '11-15',
  '16-20',
  '21+'
);

create type mentor_experience_level as enum(
  'intermediate',
  'first_level_management',
  'middle_management',
  'senior_or_executive'
);

create type volunteer_hear_about as enum(
  'linkedin',
  'university',
  'company_social_impact_team',
  'colleague',
  'dfg_member',
  'nonprofit',
  'online_ad',
  'instagram',
  'word_of_mouth',
  'bootcamp',
  'discord_or_slack',
  'unknown',
  'other'
);

create type nonprofit_hear_about as enum(
  'linkedin',
  'former_dfg_client',
  'dfg_member',
  'online_ad',
  'news_article',
  'social_media',
  'company_nonprofit_network',
  'fast_forward',
  'all_stars_helping_kids',
  'word_of_mouth',
  'other'
);

create type impact_cause as enum(
  'animals',
  'career_and_professional_development',
  'disaster_relief',
  'education',
  'environment_and_sustainability',
  'faith_and_religion',
  'health_and_medicine',
  'global_relations',
  'poverty_and_hunger',
  'senior_services',
  'justice_and_equity',
  'veterans_and_military_families',
  'other'
);

-- This table keeps track of Develop for Good batched project cycles, for example, Summer 2024. Every volunteer (among other entities)
-- is associated with a specific project cycle.
create table if not exists project_cycles(
  id uuid not null default uuid_generate_v4() primary key,
  created_at timestamptz not null default now(),
  updated_at timestamptz,
  name text not null,
  description text,
  archived boolean not null default false, -- if a project cycle is archived, actions such as exporting volunteers cannot be performed
  -- constraints
  unique (name)
);

select
  trigger_updated_at('project_cycles');

--
-- volunteers table
-- This table keeps track of all volunteers at Develop for Good. Each volunteer is associated with a specific project cycle.
create table if not exists volunteers(
  id uuid not null default uuid_generate_v4() primary key,
  created_at timestamptz not null default now(),
  updated_at timestamptz,
  project_cycle_id uuid not null references project_cycles(id) on delete cascade, -- The project cycle this volunteer is associated with
  first_name text not null,
  last_name text not null,
  email text not null,
  phone text,
  volunteer_gender gender not null default 'prefer_not_to_say' ::gender,
  volunteer_ethnicity ethnicity[] not null default '{}' ::ethnicity[],
  volunteer_age_range age_range not null default 'prefer_not_to_say' ::age_range,
  university text[] not null default '{}',
  lgbt lgbt_status not null default 'prefer_not_to_say' ::lgbt_status,
  country text not null,
  us_state text,
  fli fli_status[] not null default '{"prefer_not_to_say"}' ::fli_status[],
  student_stage student_stage not null default 'recent_graduate' ::student_stage,
  majors text[] not null default '{}',
  minors text[] not null default '{}',
  hear_about volunteer_hear_about[] not null,
  -- constraints
  unique (email)
);

select
  trigger_updated_at('volunteers');

--
-- team_roles table
-- This table records the possible roles a volunteer can take at Develop for Good. This is recorded in a table since roles may change in the future.
create table if not exists team_roles(
  id uuid not null default uuid_generate_v4() primary key,
  created_at timestamptz not null default now(),
  updated_at timestamptz,
  name text not null,
  description text not null,
  -- constraints
  unique (name)
);

select
  trigger_updated_at('team_roles');

insert into team_roles(name, description)
  values ('product_lead', 'A product lead is a senior volunteer with either experience at Develop for Good or is an exceptional candidate. They supervise 2-5 teams.'),
('product_manager', 'A product manager is a student-manager level volunteer with exceptional leadership skills.'),
('engineering_manager', 'An engineering manager is a student-manager level volunteer with exceptional engineering skills, assigned to engineering projects.'),
('design_manager', 'A design manager is a student-manager level volunteer with exceptional design skills, assigned to design projects'),
('engineer', 'An engineer volunteer is usually assigned to engineering projects'),
('designer', 'A designer volunteer is usually assigned to design projects');

--
-- volunteer_team_roles table
-- This table records which roles a volunteer is assigned during a specific project cycle
create table if not exists volunteer_team_roles(
  project_cycle_id uuid not null references project_cycles(id) on delete cascade,
  volunteer_id uuid not null references volunteers(id) on delete cascade,
  role_id uuid not null references team_roles(id) on delete cascade,
  primary key (project_cycle_id, volunteer_id, role_id) -- A volunteer can have more than one role in a project cycle, but each role they have must be unique.
);

--
-- mentors table
-- This table keeps track of all industry mentors at Develop for Good. Each mentor is associated with a specific project cycle.
create table if not exists mentors(
  id uuid not null default uuid_generate_v4() primary key,
  created_at timestamptz not null default now(),
  updated_at timestamptz,
  project_cycle_id uuid not null references project_cycles(id) on delete cascade,
  first_name text not null,
  last_name text not null,
  email text not null,
  phone text,
  company text not null,
  job_title text not null,
  country text not null,
  us_state text,
  years_experience mentor_years_experience not null,
  experience_level mentor_experience_level not null,
  prior_mentor boolean not null default false,
  prior_mentee boolean not null default false,
  prior_student boolean not null default false,
  university text[] not null default '{}',
  hear_about volunteer_hear_about[] not null,
  -- constraints
  unique (email)
);

select
  trigger_updated_at('mentors');

--
-- nonprofit_clients table
-- This table represents all nonprofit clients at Develop for Good.
create table if not exists nonprofit_clients(
  id uuid not null default uuid_generate_v4() primary key,
  created_at timestamptz not null default now(),
  updated_at timestamptz,
  project_cycle_id uuid not null references project_cycles(id) on delete cascade,
  -- a client is semi-synonmous with their representative
  representative_first_name text not null,
  representative_last_name text not null,
  representative_job_title text,
  email text not null,
  email_cc text,
  phone text not null,
  org_name text not null,
  project_name text not null,
  -- agreement_and_invoice_sent boolean not null default false,
  -- services_agreement_signature boolean not null default false,
  -- availability_confirmed boolean not null default false,
  -- invoice_paid boolean not null default false,
  org_website text,
  country_hq text,
  us_state_hq text,
  address text not null,
  size client_size not null,
  impact_causes impact_cause[] not null default '{}',
  -- constraints
  -- unique (email, project_cycle_id),
  -- unique (org_name, project_cycle_id),
  -- unique (project_name, project_cycle_id)
  unique (email, project_cycle_id, org_name, project_name)
);

select
  trigger_updated_at('nonprofit_clients');

--
-- client_volunteers join table
-- This table keeps track of all volunteers associated with a client in a given cycle.
create table if not exists client_volunteers(
  created_at timestamptz not null default now(),
  updated_at timestamptz,
  volunteer_id uuid not null references volunteers(id) on delete cascade,
  client_id uuid not null references nonprofit_clients(id) on delete cascade,
  project_cycle_id uuid not null references project_cycles(id) on delete cascade,
  currently_active boolean not null,
  primary key (volunteer_id, client_id, project_cycle_id)
);

-- This table will only be updated if a volunteer switches teams
select
  trigger_updated_at('client_volunteers');

--
-- client_mentors join table
-- This table keeps track of all client-mentor pairings in a given cycle.
create table if not exists client_mentors(
  created_at timestamptz not null default now(),
  updated_at timestamptz,
  mentor_id uuid not null references mentors(id) on delete cascade,
  client_id uuid not null references nonprofit_clients(id) on delete cascade,
  project_cycle_id uuid not null references project_cycles(id) on delete cascade,
  primary key (mentor_id, client_id, project_cycle_id)
);

--
-- volunteer_mentors join table
-- This table keeps track of all volunteer-mentor pairings in a given cycle
create table if not exists volunteer_mentors(
  mentor_id uuid not null references mentors(id) on delete cascade,
  volunteer_id uuid not null references volunteers(id) on delete cascade,
  project_cycle_id uuid not null references project_cycles(id) on delete cascade,
  primary key (mentor_id, volunteer_id, project_cycle_id)
);

create table if not exists jobs(
  id uuid not null default uuid_generate_v4() primary key,
  created_at timestamptz not null default now(),
  updated_at timestamptz,
  project_cycle_id uuid references project_cycles(id) on delete cascade,
  status job_status not null default 'pending' ::job_status,
  label text not null,
  description text,
  details jsonb not null default '{}' ::jsonb
);

select
  trigger_updated_at('jobs');

create table if not exists volunteers_exported_to_workspace(
  id uuid not null default uuid_generate_v4() primary key,
  created_at timestamptz not null default now(),
  updated_at timestamptz,
  volunteer_id uuid not null references volunteers(id) on delete cascade,
  job_id uuid not null references jobs(id) on delete cascade,
  workspace_email text not null,
  org_unit text not null,
  unique (volunteer_id, job_id)
);

select
  trigger_updated_at('volunteers_exported_to_workspace');

-- university text[] not null default '{}',
-- lgbt lgbt_status not null default 'prefer_not_to_say' ::lgbt_status,
-- country text not null,
-- us_state text,
-- fli fli_status[] not null default '{"prefer_not_to_say"}' ::fli_status[],
-- student_status student_status not null default 'recent_graduate' ::student_status,
-- majors text[] not null default '{}',
-- minors text[] not null default '{}',
-- hear_about text[] not null,
create view volunteer_details as
select
  v.id as volunteer_id,
  v.created_at,
  v.updated_at,
  v.project_cycle_id,
  pc.name as project_cycle_name,
  v.first_name,
  v.last_name,
  v.email,
  v.phone,
  v.volunteer_gender,
  v.volunteer_ethnicity,
  v.volunteer_age_range,
  v.university,
  v.lgbt,
  v.country,
  v.us_state,
  v.fli,
  v.student_stage,
  v.majors,
  v.minors,
  v.hear_about,
  vew.workspace_email,
  coalesce(json_agg(distinct jsonb_build_object('clientId', nc.id, 'orgName', nc.org_name, 'projectName', nc.project_name, 'currentlyActive', cv.currently_active)) filter (where nc.id is not null), '[]') as clients,
  coalesce(json_agg(distinct jsonb_build_object('mentorId', vm.mentor_id, 'firstName', m.first_name, 'lastName', m.last_name, 'email', m.email, 'phone', m.phone, 'company', m.company, 'jobTitle', m.job_title)) filter (where vm.mentor_id is not null), '[]') as mentors,
  coalesce(json_agg(distinct jsonb_build_object('roleId', vtr.role_id, 'name', tr.name, 'description', tr.description)) filter (where vtr.role_id is not null), '[]') as roles
from
  volunteers v
  left join client_volunteers cv on v.id = cv.volunteer_id
  left join nonprofit_clients nc on cv.client_id = nc.id
  left join volunteer_mentors vm on v.id = vm.volunteer_id
  left join mentors m on vm.mentor_id = m.id
  left join volunteer_team_roles vtr on v.id = vtr.volunteer_id
  left join team_roles tr on vtr.role_id = tr.id
  left join project_cycles pc on pc.id = v.project_cycle_id
  left join volunteers_exported_to_workspace vew on v.id = vew.volunteer_id
group by
  v.id,
  vew.workspace_email,
  pc.name;

create view nonprofit_client_details as
select
  nc.id as client_id,
  nc.created_at,
  nc.updated_at,
  nc.project_cycle_id,
  pc.name as project_cycle_name,
  nc.representative_first_name,
  nc.representative_last_name,
  nc.representative_job_title,
  nc.email,
  nc.email_cc,
  nc.phone,
  nc.org_name,
  nc.project_name,
  nc.impact_causes,
  nc.org_website,
  nc.country_hq,
  nc.us_state_hq,
  nc.address,
  nc.size,
  coalesce(json_agg(distinct jsonb_build_object('id', v.id, 'first_name', v.first_name, 'last_name', v.last_name, 'email', v.email, 'phone', v.phone, 'volunteer_gender', v.volunteer_gender, 'volunteer_ethnicity', v.volunteer_ethnicity, 'volunteer_age_range', v.volunteer_age_range)) filter (where cv.volunteer_id is not null), '[]') as volunteers,
  coalesce(json_agg(distinct jsonb_build_object('id', m.id, 'first_name', m.first_name, 'last_name', m.last_name, 'email', m.email, 'phone', m.phone, 'company', m.company, 'job_title', m.job_title)) filter (where cm.mentor_id is not null), '[]') as mentors
from
  nonprofit_clients nc
  left join client_volunteers cv on nc.id = cv.client_id
  left join volunteers v on cv.volunteer_id = v.id
  left join client_mentors cm on nc.id = cm.client_id
  left join mentors m on cm.mentor_id = m.id
  left join project_cycles pc on pc.id = nc.project_cycle_id
group by
  nc.id,
  pc.name;

create view mentor_details as
select
  m.id as mentor_id,
  m.created_at,
  m.updated_at,
  m.project_cycle_id,
  pc.name as project_cycle_name,
  m.first_name,
  m.last_name,
  m.email,
  m.phone,
  m.company,
  m.job_title,
  m.country,
  m.us_state,
  m.years_experience,
  m.experience_level,
  m.prior_mentor,
  m.prior_mentee,
  m.prior_student,
  m.university,
  m.hear_about,
  coalesce(json_agg(distinct jsonb_build_object('volunteer_id', vm.volunteer_id, 'email', v.email, 'name', v.first_name || ' ' || v.last_name)) filter (where vm.volunteer_id is not null), '[]') as volunteers,
  coalesce(json_agg(distinct jsonb_build_object('client_id', cm.client_id, 'org_name', nc.org_name, 'project_name', nc.project_name)) filter (where cm.client_id is not null), '[]') as clients
from
  mentors m
  left join volunteer_mentors vm on m.id = vm.mentor_id
  left join volunteers v on vm.volunteer_id = v.id
  left join client_mentors cm on m.id = cm.mentor_id
  left join nonprofit_clients nc on cm.client_id = nc.id
  left join project_cycles pc on m.project_cycle_id = pc.id
group by
  m.id,
  pc.name;

create view exported_volunteer_details as
select
  ev.id,
  ev.created_at,
  ev.updated_at,
  ev.volunteer_id,
  ev.workspace_email,
  ev.org_unit,
  j.id as job_id,
  j.project_cycle_id,
  j.status
from
  volunteers_exported_to_workspace ev
  left join jobs j on ev.job_id = j.id
group by
  ev.id,
  j.id;

