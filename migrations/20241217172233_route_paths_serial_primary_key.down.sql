ALTER TABLE route_paths DROP COLUMN id;
ALTER TABLE stops DROP COLUMN id;

ALTER TABLE route_paths ADD PRIMARY KEY (route_code); 
ALTER TABLE stops ADD PRIMARY KEY (stop_code); 
