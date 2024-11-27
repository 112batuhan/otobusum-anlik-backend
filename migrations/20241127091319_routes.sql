CREATE TABLE routes(
  id SERIAL PRIMARY KEY,
  agency_id INTEGER,
  route_short_name TEXT,
  route_long_name TEXT,
  route_type INTEGER,
  route_desc TEXT,
  route_code TEXT
);
