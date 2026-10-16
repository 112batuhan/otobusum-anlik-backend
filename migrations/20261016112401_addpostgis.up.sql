ALTER TABLE stops
  ADD COLUMN location geometry(Point, 4326);

UPDATE stops
SET location = ST_SetSRID(
    ST_MakePoint(lng, lat),
    4326
)
WHERE lat IS NOT NULL
  AND lng IS NOT NULL;

ALTER TABLE stops
ALTER COLUMN location SET NOT NULL;

CREATE INDEX idx_stops_location
ON stops
USING GIST (location);
