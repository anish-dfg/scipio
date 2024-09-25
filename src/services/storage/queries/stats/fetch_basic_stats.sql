select
  (
    select
      count(*)
    from
      volunteers
    where
      volunteers.project_cycle_id = $1) as num_volunteers,
(
    select
      count(*)
    from
      mentors
    where
      mentors.project_cycle_id = $1) as num_mentors,
(
    select
      count(*)
    from
      nonprofit_clients
    where
      nonprofit_clients.project_cycle_id = $1) as num_nonprofits;

