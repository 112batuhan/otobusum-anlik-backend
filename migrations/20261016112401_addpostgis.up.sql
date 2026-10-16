ALTER TABLE stops
  ADD COLUMN IF NOT EXISTS location geometry(Point, 4326);

ALTER TABLE route_paths
  ADD COLUMN IF NOT EXISTS path_geom geometry(LineString, 4326);

UPDATE stops
SET location = ST_SetSRID(
    ST_MakePoint(lng, lat),
    4326
) WHERE lat IS NOT NULL
    AND lng IS NOT NULL;

UPDATE route_paths
SET path_geom = ST_SetSRID(
    ST_MakeLine(
        ARRAY(
            SELECT ST_MakePoint(
                (point->>'lng')::double precision,
                (point->>'lat')::double precision
            )
            FROM jsonb_array_elements(path::jsonb) WITH ORDINALITY AS p(point, ord)
            ORDER BY ord
        )
    ),
    4326
)
WHERE path IS NOT NULL
  AND jsonb_typeof(path::jsonb) = 'array';

ALTER TABLE stops
ALTER COLUMN location SET NOT NULL;

ALTER TABLE route_paths
DROP COLUMN path;

ALTER TABLE route_paths
RENAME COLUMN path_geom TO path;

CREATE INDEX idx_stops_location
ON stops
USING GIST (location);

CREATE INDEX idx_route_paths_path_geom
ON route_paths
USING GIST (path);
