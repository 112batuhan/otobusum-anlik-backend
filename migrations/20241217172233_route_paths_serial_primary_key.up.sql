ALTER TABLE route_paths DROP CONSTRAINT route_paths_pkey;
ALTER TABLE stops DROP CONSTRAINT stops_pkey;

ALTER TABLE route_paths ADD COLUMN id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY;
ALTER TABLE stops ADD COLUMN id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY;
