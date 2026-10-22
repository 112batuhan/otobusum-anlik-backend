CREATE TABLE IF NOT EXISTS route_paths(
  id SERIAL PRIMARY KEY,
  route_code TEXT NOT NULL,
  path geometry(LineString, 4326) NOT NULL,
  city TEXT NOT NULL DEFAULT 'istanbul'
);

CREATE UNIQUE INDEX IF NOT EXISTS id_code_index_paths ON route_paths (route_code, city);
CREATE INDEX IF NOT EXISTS idx_route_paths_path_geom ON route_paths USING GIST (path);
