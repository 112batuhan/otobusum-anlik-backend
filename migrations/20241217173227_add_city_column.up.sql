ALTER TABLE lines ADD city TEXT NOT NULL DEFAULT('istanbul');
ALTER TABLE routes ADD city TEXT NOT NULL DEFAULT('istanbul');
ALTER TABLE route_paths ADD city TEXT NOT NULL DEFAULT('istanbul');
ALTER TABLE stops ADD city TEXT NOT NULL DEFAULT('istanbul');
ALTER TABLE line_stops ADD city TEXT NOT NULL DEFAULT('istanbul');

CREATE UNIQUE INDEX id_code_index_lines ON lines (code, city);
CREATE UNIQUE INDEX id_code_index_routes ON routes (route_code, city);
CREATE UNIQUE INDEX id_code_index_paths ON route_paths (route_code, city);
CREATE UNIQUE INDEX id_code_index_stops ON stops (stop_code, city);
CREATE UNIQUE INDEX id_code_index_line_stops ON line_stops (stop_code, city);
