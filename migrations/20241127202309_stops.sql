CREATE TABLE IF NOT EXISTS stops(
  id SERIAL PRIMARY KEY,
  stop_code INTEGER NOT NULL,
  name TEXT NOT NULL,
  lng DOUBLE PRECISION NOT NULL,
  lat DOUBLE PRECISION NOT NULL,
  province TEXT,
  location geometry(Point, 4326) NOT NULL,
  city TEXT NOT NULL DEFAULT 'istanbul'
);

CREATE UNIQUE INDEX IF NOT EXISTS id_code_index_stops ON stops (stop_code, city);
CREATE INDEX IF NOT EXISTS idx_stops_location ON stops USING GIST (location);
