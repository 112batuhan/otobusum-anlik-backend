-- Add up migration script here
CREATE UNIQUE INDEX route_paths_route_code_city ON route_paths (route_code, city);
