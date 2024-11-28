CREATE TABLE line_stops(
  id SERIAL PRIMARY KEY,
  line_code TEXT NOT NULL,
  stop_code INTEGER NOT NULL
);
