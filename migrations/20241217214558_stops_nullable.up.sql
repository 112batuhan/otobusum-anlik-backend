ALTER TABLE stops ALTER COLUMN province DROP NOT NULL;
ALTER TABLE stops ALTER COLUMN smart DROP NOT NULL;
ALTER TABLE stops ALTER COLUMN physical DROP NOT NULL;
ALTER TABLE stops ALTER COLUMN stop_type DROP NOT NULL;
ALTER TABLE stops ALTER COLUMN disabled_can_use DROP NOT NULL;

ALTER TABLE stops DROP COLUMN direction;

ALTER TABLE line_stops ADD route_code TEXT;
CREATE UNIQUE INDEX id_code_index_line_route_stops ON line_stops (route_code, stop_code, city);
