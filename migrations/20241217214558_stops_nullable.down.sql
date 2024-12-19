ALTER TABLE stops ALTER COLUMN province DROP NOT NULL;
ALTER TABLE stops ALTER COLUMN smart DROP NOT NULL;
ALTER TABLE stops ALTER COLUMN physical DROP NOT NULL;
ALTER TABLE stops ALTER COLUMN stop_type DROP NOT NULL;
ALTER TABLE stops ALTER COLUMN disabled_can_use DROP NOT NULL;

ALTER TABLE stops ADD COLUMN direction TEXT;

ALTER TABLE line_stops DROP COLUMN route_code;
-- ALTER TABLE line_stops DROP INDEX id_code_index_line_route_stops;
