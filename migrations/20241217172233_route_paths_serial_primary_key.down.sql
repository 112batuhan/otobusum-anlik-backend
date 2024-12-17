ALTER TABLE route_paths DROP COLUMN id;
ALTER TABLE route_paths ADD PRIMARY KEY (route_code); 
