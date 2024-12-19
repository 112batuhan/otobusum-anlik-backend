ALTER TABLE stops ALTER COLUMN province SET NOT NULL;
ALTER TABLE stops ALTER COLUMN smart SET NOT NULL;
ALTER TABLE stops ALTER COLUMN physical SET NOT NULL;
ALTER TABLE stops ALTER COLUMN stop_type SET NOT NULL;
ALTER TABLE stops ALTER COLUMN disabled_can_use SET NOT NULL;

ALTER TABLE stops DROP COLUMN direction;

ALTER TABLE line_stops ADD route_code TEXT;
CREATE UNIQUE INDEX id_code_index_line_route_stops ON line_stops (route_code, stop_code, city);
