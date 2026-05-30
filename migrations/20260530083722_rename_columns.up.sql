ALTER TABLE routes RENAME route_short_name TO code;
ALTER TABLE routes RENAME route_long_name TO title;
ALTER TABLE routes RENAME route_desc TO description;
ALTER TABLE routes RENAME route_type TO type;

ALTER TABLE stops RENAME stop_name TO name;
ALTER TABLE stops RENAME stop_type TO type;
ALTER TABLE stops RENAME x_coord TO lng;
ALTER TABLE stops RENAME y_coord TO lat;

ALTER TABLE route_paths RENAME route_path TO path;
