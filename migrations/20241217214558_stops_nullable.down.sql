ALTER TABLE stops ALTER COLUMN province SET NOT NULL;
ALTER TABLE stops ALTER COLUMN smart SET NOT NULL;
ALTER TABLE stops ALTER COLUMN physical SET NOT NULL;
ALTER TABLE stops ALTER COLUMN stop_type SET NOT NULL;
ALTER TABLE stops ALTER COLUMN disabled_can_use SET NOT NULL;

ALTER TABLE stops ADD COLUMN direction TEXT;

ALTER TABLE line_stops DROP COLUMN route_code;
-- ALTER TABLE line_stops DROP INDEX id_code_index_line_route_stops;
