-- Add up migration script here
CREATE TABLE IF NOT EXISTS routes(
  id SERIAL PRIMARY KEY,
  agency_id INTEGER,
  code TEXT NOT NULL,
  title TEXT NOT NULL,
  type INTEGER,
  description TEXT,
  route_code TEXT NOT NULL,
  city TEXT NOT NULL DEFAULT 'istanbul'
);

CREATE UNIQUE INDEX IF NOT EXISTS id_code_index_routes ON routes (route_code, city);

CREATE TABLE IF NOT EXISTS lines(
  id SERIAL PRIMARY KEY,
  code TEXT NOT NULL,
  title TEXT NOT NULL,
  city TEXT NOT NULL DEFAULT 'istanbul'
);

CREATE UNIQUE INDEX IF NOT EXISTS id_code_index_lines ON lines (code, city);

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

CREATE TABLE IF NOT EXISTS line_stops(
  id SERIAL PRIMARY KEY,
  line_code TEXT NOT NULL,
  stop_code INTEGER NOT NULL,
  route_code TEXT NOT NULL,
  stop_order INTEGER NOT NULL,
  city TEXT NOT NULL DEFAULT 'istanbul'
);

CREATE UNIQUE INDEX IF NOT EXISTS id_code_index_line_route_stops ON line_stops (route_code, stop_code, city);

CREATE TABLE IF NOT EXISTS route_paths(
  id SERIAL PRIMARY KEY,
  route_code TEXT NOT NULL,
  path geometry(LineString, 4326) NOT NULL,
  city TEXT NOT NULL DEFAULT 'istanbul'
);

CREATE UNIQUE INDEX IF NOT EXISTS id_code_index_paths ON route_paths (route_code, city);
CREATE INDEX IF NOT EXISTS idx_route_paths_path_geom ON route_paths USING GIST (path);

CREATE TABLE IF NOT EXISTS timetable (
  route_code TEXT NOT NULL,
  city TEXT NOT NULL,
  sunday TIME[],
  monday TIME[],
  tuesday TIME[],
  wednesday TIME[],
  thursday TIME[],
  friday TIME[],
  saturday TIME[]
);

