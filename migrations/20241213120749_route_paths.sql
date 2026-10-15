CREATE TABLE IF NOT EXISTS route_paths(
  id SERIAL PRIMARY KEY,
  route_code TEXT NOT NULL,
  path JSONB NOT NULL,
  city TEXT NOT NULL
);

CREATE UNIQUE INDEX IF NOT EXISTS id_code_index_paths ON route_paths (route_code, city);
