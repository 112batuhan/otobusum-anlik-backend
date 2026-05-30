ALTER TABLE routes RENAME code TO route_short_name;
ALTER TABLE routes RENAME title TO route_long_name;
ALTER TABLE routes RENAME description TO route_desc;
ALTER TABLE routes RENAME type TO route_type;

ALTER TABLE stops RENAME name TO stop_name;
ALTER TABLE stops RENAME type TO stop_type;
ALTER TABLE stops RENAME lng TO x_coord;
ALTER TABLE stops RENAME lat TO y_coord;

ALTER TABLE route_paths RENAME path TO route_path;
