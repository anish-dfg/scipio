update
  jobs
set
  status = $2,
  details = case when $3 is not null then
    jsonb_set(details, '{error}', to_jsonb($3::text), true)
  when $3 is null
    and details ? 'error' then
    details - 'error'
  else
    details
  end
where
  id = $1;

