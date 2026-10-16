ALTER TABLE stops
  DROP COLUMN location;

ALTER TABLE route_paths
  DROP COLUMN path;

DROP INDEX IF EXISTS idx_stops_location;
DROP INDEX IF EXISTS idx_route_paths_path;
