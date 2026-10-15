CREATE TABLE IF NOT EXISTS line_stops(
  id SERIAL PRIMARY KEY,
  line_code TEXT NOT NULL,
  stop_code INTEGER NOT NULL,
  route_code TEXT NOT NULL,
  stop_order INTEGER NOT NULL,
  city TEXT NOT NULL
);

CREATE UNIQUE INDEX IF NOT EXISTS id_code_index_line_route_stops ON line_stops (route_code, stop_code, city);
