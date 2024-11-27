CREATE TABLE routes(
  id SERIAL PRIMARY KEY,
  agencyid INTEGER NOT NULL,
  routeshortname TEXT NOT NULL,
  routelongname TEXT NOT NULL,
  routetype TEXT NOT NULL,
  routedesc TEXT NOT NULL,
  routecode TEXT,
  rank INTEGER NOT NULL
);
