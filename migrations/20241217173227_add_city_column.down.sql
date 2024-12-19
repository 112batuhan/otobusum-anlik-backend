ALTER TABLE lines DROP city;
ALTER TABLE routes DROP city;
ALTER TABLE route_paths DROP city;
ALTER TABLE stops DROP city;
ALTER TABLE line_stops DROP city;

-- ALTER TABLE lines DROP INDEX id_code_index_lines;
-- ALTER TABLE routes DROP INDEX id_code_index_routes;
-- ALTER TABLE route_paths DROP INDEX id_code_index_route_paths;
-- ALTER TABLE stops DROP INDEX id_code_index_stops;
