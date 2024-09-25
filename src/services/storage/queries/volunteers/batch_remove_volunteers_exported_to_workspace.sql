delete from volunteers_exported_to_workspace
where volunteer_id = any ($1);

