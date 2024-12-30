ALTER TABLE line_stops
  ALTER COLUMN route_code DROP NOT NULL,
  ALTER COLUMN line_code DROP NOT NULL,
  ALTER COLUMN stop_code DROP NOT NULL-- Add down migration script here
