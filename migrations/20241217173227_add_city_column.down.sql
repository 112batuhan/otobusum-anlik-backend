ALTER TABLE lines DROP city;
ALTER TABLE routes DROP city;
ALTER TABLE route_paths DROP city;
ALTER TABLE stops DROP city;
ALTER TABLE line_stops DROP city;

ALTER TABLE lines DROP CONSTRAINT id_codes;
ALTER TABLE routes DROP CONSTRAINT id_codes;
ALTER TABLE route_paths DROP CONSTRAINT id_codes;
ALTER TABLE stops DROP CONSTRAINT id_codes;
ALTER TABLE line_stops DROP CONSTRAINT id_codes;
