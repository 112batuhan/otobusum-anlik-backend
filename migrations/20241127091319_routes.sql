CREATE TABLE IF NOT EXISTS routes(
  id SERIAL PRIMARY KEY,
  agency_id INTEGER,
  code TEXT NOT NULL,
  title TEXT NOT NULL,
  type INTEGER,
  description TEXT,
  route_code TEXT NOT NULL,
  city TEXT NOT NULL
);

CREATE UNIQUE INDEX IF NOT EXISTS id_code_index_routes ON routes (route_code, city);
