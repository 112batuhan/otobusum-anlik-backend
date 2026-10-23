-- Add down migration script here
DROP TABLE routes;

DROP INDEX IF EXISTS id_code_index_routes;

DROP TABLE lines;

DROP INDEX IF EXISTS id_code_index_lines;

DROP TABLE stops;

DROP INDEX IF EXISTS idx_stops_location;
DROP INDEX IF EXISTS id_code_index_stops;

DROP TABLE line_stops;

DROP INDEX IF EXISTS id_code_index_line_route_stops;

DROP TABLE route_paths;

DROP INDEX IF EXISTS id_code_index_paths;
DROP INDEX IF EXISTS idx_route_paths_path_geom;

DROP TABLE IF EXISTS timetable;
