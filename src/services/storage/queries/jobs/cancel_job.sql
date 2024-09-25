update
  jobs
set
  status = 'cancelled'
where
  id = $1
  and status = 'pending';

