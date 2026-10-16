ALTER TABLE stops
  DROP COLUMN location;

DROP INDEX IF EXISTS idx_stops_location;
