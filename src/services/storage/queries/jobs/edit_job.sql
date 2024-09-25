update
  jobs
set
  label = case when $2 is not null then
    $2
  else
    label
  end,
  description = $3
where
  id = $1;

