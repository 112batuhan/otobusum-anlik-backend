CREATE TABLE stops(
  stop_code INTEGER PRIMARY KEY NOT NULL,
  stop_name TEXT NOT NULL,
  x_coord DOUBLE PRECISION NOT NULL,
  y_coord DOUBLE PRECISION NOT NULL,
  province TEXT NOT NULL,
  direction TEXT NOT NULL,
  smart Text NOT NULL,
  physical Text,
  stop_type Text NOT NULL,
  disabled_can_use Text NOT NULL
);
